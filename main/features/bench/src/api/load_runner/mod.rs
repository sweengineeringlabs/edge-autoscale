//! Trait for driving load against the system under test.

pub(crate) mod tokio_load_runner;

use async_trait::async_trait;

use crate::api::load_report::StepResult;

/// Runs one concurrency step and returns the measured results.
///
/// The concrete implementation is [`crate::core::load_runner::TokioLoadRunner`].
#[async_trait]
pub trait LoadRunner: Send + Sync {
    /// Drive `concurrency` concurrent tasks for the configured duration and
    /// return the aggregate latency histogram and throughput for this step.
    async fn run_step(&self, concurrency: usize) -> StepResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FixedRunner { concurrency_echo: bool }

    #[async_trait::async_trait]
    impl LoadRunner for FixedRunner {
        async fn run_step(&self, concurrency: usize) -> StepResult {
            StepResult {
                concurrency: if self.concurrency_echo { concurrency } else { 0 },
                rps: 1000.0,
                p50_ms: 0.5,
                p95_ms: 1.0,
                p99_ms: 2.0,
                p99_9_ms: 4.0,
                error_count: 0,
            }
        }
    }

    #[tokio::test]
    async fn test_run_step_returns_result_with_matching_concurrency() {
        let r = FixedRunner { concurrency_echo: true };
        let s = r.run_step(8).await;
        assert_eq!(s.concurrency, 8);
    }

    #[tokio::test]
    async fn test_run_step_result_has_positive_rps() {
        let r = FixedRunner { concurrency_echo: true };
        let s = r.run_step(1).await;
        assert!(s.rps > 0.0);
    }
}
