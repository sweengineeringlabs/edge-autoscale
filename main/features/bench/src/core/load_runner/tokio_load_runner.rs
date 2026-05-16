//! Tokio-based concurrent load runner.

use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;

use crate::api::bench_handler::BenchHandler;
use crate::api::load_report::StepResult;
use crate::api::load_runner::LoadRunner;

/// Drives `concurrency` concurrent Tokio tasks for a configurable window,
/// records per-call latency for successful calls, and returns aggregated
/// percentiles and throughput.
pub(crate) struct TokioLoadRunner {
    pub(crate) handler:            Arc<dyn BenchHandler>,
    pub(crate) warmup_secs:        u64,
    pub(crate) step_duration_secs: u64,
}

#[async_trait]
impl LoadRunner for TokioLoadRunner {
    async fn run_step(&self, concurrency: usize) -> StepResult {
        self.warmup(concurrency).await;
        self.measure(concurrency).await
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
                            Ok(())  => durations.push(t0.elapsed()),
                            Err(_)  => errors += 1,
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
            rps:      n as f64 / self.step_duration_secs as f64,
            p50_ms:   pct(50.0),
            p95_ms:   pct(95.0),
            p99_ms:   pct(99.0),
            p99_9_ms: pct(99.9),
            error_count,
        }
    }
}
