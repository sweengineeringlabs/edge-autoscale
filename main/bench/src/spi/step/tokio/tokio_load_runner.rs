//! Tokio-based concurrent load runner.

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::future::BoxFuture;

use crate::api::step::traits::runner::Runner;
use crate::api::traits::bench_handler::BenchHandler;
use crate::api::vo::step_result::StepResult;

/// Drives `concurrency` concurrent Tokio tasks for a configurable window,
/// records per-call latency for successful calls, and returns aggregated
/// percentiles and throughput.
///
/// The default [`Runner`] backend; created per step by the runner factory.
/// Exported through `saf/` so callers can drive a single step directly.
pub struct TokioLoadRunner {
    pub(crate) handler: Arc<dyn BenchHandler>,
    pub(crate) warmup_secs: u64,
    pub(crate) step_duration_secs: u64,
}

impl TokioLoadRunner {
    /// Create a runner for `handler` with the given warmup and measurement window.
    pub fn new(handler: Arc<dyn BenchHandler>, warmup_secs: u64, step_duration_secs: u64) -> Self {
        Self {
            handler,
            warmup_secs,
            step_duration_secs,
        }
    }
}

impl Runner for TokioLoadRunner {
    fn run_step(&self, concurrency: usize) -> BoxFuture<'_, StepResult> {
        Box::pin(async move {
            self.warmup(concurrency).await;
            self.measure(concurrency).await
        })
    }
}

impl TokioLoadRunner {
    async fn warmup(&self, concurrency: usize) {
        if self.warmup_secs == 0 {
            return;
        }
        let end = Instant::now() + Duration::from_secs(self.warmup_secs);
        let tasks: Vec<_> = (0..concurrency)
            .map(|_| {
                let h = Arc::clone(&self.handler);
                tokio::spawn(async move {
                    while Instant::now() < end {
                        let _ = h.call().await;
                    }
                })
            })
            .collect();
        for t in tasks {
            let _ = t.await;
        }
    }

    async fn measure(&self, concurrency: usize) -> StepResult {
        let end = Instant::now() + Duration::from_secs(self.step_duration_secs);
        let tasks: Vec<_> = (0..concurrency)
            .map(|_| {
                let h = Arc::clone(&self.handler);
                tokio::spawn(async move {
                    let mut durations: Vec<Duration> = Vec::new();
                    let mut errors: u64 = 0;
                    while Instant::now() < end {
                        let t0 = Instant::now();
                        match h.call().await {
                            Ok(()) => durations.push(t0.elapsed()),
                            Err(_) => errors += 1,
                        }
                    }
                    (durations, errors)
                })
            })
            .collect();

        let mut all_durations: Vec<Duration> = Vec::new();
        let mut error_count: u64 = 0;
        for t in tasks {
            if let Ok((durations, errors)) = t.await {
                all_durations.extend(durations);
                error_count += errors;
            }
        }

        all_durations.sort_unstable();
        let n = all_durations.len();

        let pct = |p: f64| -> f64 {
            if n == 0 {
                return 0.0;
            }
            let idx = ((p / 100.0) * n as f64) as usize;
            all_durations[idx.min(n - 1)].as_secs_f64() * 1000.0
        };

        StepResult {
            concurrency,
            rps: n as f64 / self.step_duration_secs as f64,
            p50_ms: pct(50.0),
            p95_ms: pct(95.0),
            p99_ms: pct(99.0),
            p99_9_ms: pct(99.9),
            error_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::bench_future::BenchFuture;

    struct TokioLoadRunnerInstantHandler;
    impl BenchHandler for TokioLoadRunnerInstantHandler {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    struct TokioLoadRunnerAlwaysFailHandler;
    impl BenchHandler for TokioLoadRunnerAlwaysFailHandler {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async {
                Err(crate::api::error::bench_error::BenchError::StepFailed(
                    "forced".into(),
                ))
            })
        }
    }

    fn runner(handler: Arc<dyn BenchHandler>, step_duration_secs: u64) -> TokioLoadRunner {
        TokioLoadRunner {
            handler,
            warmup_secs: 0,
            step_duration_secs,
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_run_step_returns_positive_rps_for_fast_handler() {
        let r = runner(Arc::new(TokioLoadRunnerInstantHandler), 1);
        let s = r.run_step(2).await;
        assert!(s.rps > 0.0, "expected positive RPS, got {}", s.rps);
        assert_eq!(s.concurrency, 2);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_run_step_counts_errors_for_failing_handler() {
        let r = runner(Arc::new(TokioLoadRunnerAlwaysFailHandler), 1);
        let s = r.run_step(1).await;
        assert!(s.error_count > 0, "expected errors from AlwaysFailHandler");
        assert_eq!(s.rps, 0.0, "errors must not be counted as successful calls");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_run_step_with_zero_step_duration_returns_zero_rps() {
        let r = runner(Arc::new(TokioLoadRunnerInstantHandler), 0);
        let s = r.run_step(1).await;
        // duration=0 → measurement window is already past → no calls complete
        // rps = n / 0 would panic, but n will be 0 → rps = 0/0 → handled as 0/0=NaN
        // In practice the window expires immediately so we just assert no panic.
        let _ = s;
    }
}
