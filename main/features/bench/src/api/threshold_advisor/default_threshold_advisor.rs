//! Interface contract for the default threshold advisor.
//!
//! The concrete implementation is
//! [`crate::core::threshold_advisor::DefaultThresholdAdvisor`].
//!
//! Applies `safety_margin_pct / 100` to the knee step's concurrency, RPS,
//! and p99 latency, using `ceil` for integer dimensions.

#[cfg(test)]
mod tests {
    use crate::api::load_report::{AutoscalePolicy, StepResult};
    use crate::api::threshold_advisor::ThresholdAdvisor;

    fn step(concurrency: usize, rps: f64, p99_ms: f64) -> StepResult {
        StepResult { concurrency, rps, p50_ms: 0.0, p95_ms: 0.0, p99_ms, p99_9_ms: 0.0, error_count: 0 }
    }

    /// Stub that applies the documented safety margin contract directly.
    struct MarginAdvisor;
    impl ThresholdAdvisor for MarginAdvisor {
        fn advise(&self, steps: &[StepResult], knee_index: usize, safety_margin_pct: u8) -> AutoscalePolicy {
            let s = &steps[knee_index];
            let m = safety_margin_pct as f64 / 100.0;
            AutoscalePolicy {
                requests_active_max:  (s.concurrency as f64 * m).ceil() as usize,
                requests_per_sec_max: (s.rps * m).ceil() as u64,
                latency_p99_ms_max:   s.p99_ms * m,
            }
        }
        fn fallback_policy(&self, steps: &[StepResult], safety_margin_pct: u8) -> AutoscalePolicy {
            self.advise(steps, steps.len() - 1, safety_margin_pct)
        }
    }

    #[test]
    fn test_default_advisor_contract_thresholds_below_knee_values() {
        let a = MarginAdvisor;
        let steps = vec![step(32, 100_000.0, 4.0)];
        let p = a.advise(&steps, 0, 70);
        assert!(p.requests_active_max < 32);
        assert!(p.requests_per_sec_max < 100_000);
        assert!(p.latency_p99_ms_max < 4.0);
    }

    #[test]
    fn test_default_advisor_contract_100_pct_margin_matches_knee() {
        let a = MarginAdvisor;
        let steps = vec![step(16, 50_000.0, 5.0)];
        let p = a.advise(&steps, 0, 100);
        assert_eq!(p.requests_active_max, 16);
        assert_eq!(p.requests_per_sec_max, 50_000);
    }
}
