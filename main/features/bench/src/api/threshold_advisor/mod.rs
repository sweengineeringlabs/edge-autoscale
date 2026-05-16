//! Trait for translating a knee step into autoscale thresholds.

pub(crate) mod default_threshold_advisor;

use crate::api::load_report::{AutoscalePolicy, StepResult};

/// Computes recommended autoscale thresholds from the knee step.
///
/// The concrete implementation is
/// [`crate::core::threshold_advisor::DefaultThresholdAdvisor`].
pub trait ThresholdAdvisor: Send + Sync {
    /// Derive thresholds from `steps[knee_index]`, applying `safety_margin_pct`.
    fn advise(
        &self,
        steps: &[StepResult],
        knee_index: usize,
        safety_margin_pct: u8,
    ) -> AutoscalePolicy;

    /// Fallback when no knee was detected — uses the final step.
    fn fallback_policy(&self, steps: &[StepResult], safety_margin_pct: u8) -> AutoscalePolicy;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HalfMarginAdvisor;

    impl ThresholdAdvisor for HalfMarginAdvisor {
        fn advise(&self, steps: &[StepResult], knee_index: usize, _margin: u8) -> AutoscalePolicy {
            AutoscalePolicy {
                requests_active_max: steps[knee_index].concurrency / 2,
                requests_per_sec_max: (steps[knee_index].rps / 2.0) as u64,
                latency_p99_ms_max: steps[knee_index].p99_ms / 2.0,
            }
        }
        fn fallback_policy(&self, steps: &[StepResult], margin: u8) -> AutoscalePolicy {
            self.advise(steps, steps.len() - 1, margin)
        }
    }

    fn step(concurrency: usize, rps: f64, p99_ms: f64) -> StepResult {
        StepResult {
            concurrency,
            rps,
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms,
            p99_9_ms: 0.0,
            error_count: 0,
        }
    }

    #[test]
    fn test_advise_derives_policy_from_knee_step() {
        let a = HalfMarginAdvisor;
        let steps = vec![step(1, 1000.0, 2.0), step(2, 1800.0, 8.0)];
        let p = a.advise(&steps, 1, 70);
        assert_eq!(p.requests_active_max, 1);
        assert_eq!(p.requests_per_sec_max, 900);
        assert!((p.latency_p99_ms_max - 4.0).abs() < 0.01);
    }

    #[test]
    fn test_fallback_policy_uses_last_step() {
        let a = HalfMarginAdvisor;
        let steps = vec![step(1, 500.0, 1.0), step(4, 1200.0, 3.0)];
        let p = a.fallback_policy(&steps, 70);
        assert_eq!(p.requests_active_max, 2);
    }
}
