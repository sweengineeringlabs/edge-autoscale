//! Interface contract for the Tokio-based load runner.
//!
//! The concrete implementation is
//! [`crate::core::load_runner::TokioLoadRunner`].
//!
//! Spawns N concurrent Tokio tasks per concurrency step, records
//! per-call latency for successful calls, and computes p50/p95/p99/p99.9
//! from a sorted Vec of `Duration` values.

#[cfg(test)]
mod tests {
    use crate::api::load_report::StepResult;
    use crate::api::load_runner::LoadRunner;

    /// Stub that returns a step with RPS = concurrency * 100.
    struct ScaledRunner;

    #[async_trait::async_trait]
    impl LoadRunner for ScaledRunner {
        async fn run_step(&self, concurrency: usize) -> StepResult {
            StepResult {
                concurrency,
                rps: concurrency as f64 * 100.0,
                p50_ms: 0.1,
                p95_ms: 0.5,
                p99_ms: 1.0,
                p99_9_ms: 2.0,
                error_count: 0,
            }
        }
    }

    #[tokio::test]
    async fn test_load_runner_contract_rps_scales_with_concurrency() {
        let r = ScaledRunner;
        let s1 = r.run_step(1).await;
        let s4 = r.run_step(4).await;
        assert!(s4.rps > s1.rps, "higher concurrency must produce higher RPS");
    }

    #[tokio::test]
    async fn test_load_runner_contract_concurrency_preserved_in_result() {
        let r = ScaledRunner;
        let s = r.run_step(16).await;
        assert_eq!(s.concurrency, 16);
    }
}
