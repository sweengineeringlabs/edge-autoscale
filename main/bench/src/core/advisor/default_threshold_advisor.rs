//! Default threshold advisor — applies a safety margin to knee step values.

use crate::api::advisor::traits::threshold_advisor::ThresholdAdvisor;
use crate::api::vo::{autoscale_policy::AutoscalePolicy, step_result::StepResult};

/// Multiplies every knee-step metric by `safety_margin_pct / 100`.
///
/// Uses `ceil` for integer dimensions (`requests_active_max`,
/// `requests_per_sec_max`) so the threshold is never below the margin.
pub(crate) struct DefaultThresholdAdvisor;

impl DefaultThresholdAdvisor {
    fn apply_margin(s: &StepResult, safety_margin_pct: u8) -> AutoscalePolicy {
        let m = safety_margin_pct as f64 / 100.0;
        AutoscalePolicy {
            requests_active_max: (s.concurrency as f64 * m).ceil() as usize,
            requests_per_sec_max: (s.rps * m).ceil() as u64,
            latency_p99_ms_max: s.p99_ms * m,
        }
    }
}

impl ThresholdAdvisor for DefaultThresholdAdvisor {
    fn advise(
        &self,
        steps: &[StepResult],
        knee_index: usize,
        safety_margin_pct: u8,
    ) -> AutoscalePolicy {
        Self::apply_margin(&steps[knee_index], safety_margin_pct)
    }

    fn fallback_policy(&self, steps: &[StepResult], safety_margin_pct: u8) -> AutoscalePolicy {
        // Safety: callers guarantee `steps` is non-empty (BenchRunner checks concurrency_steps).
        // If steps is empty, return a zero-policy rather than panicking.
        match steps.last() {
            Some(s) => Self::apply_margin(s, safety_margin_pct),
            None => AutoscalePolicy {
                requests_active_max: 0,
                requests_per_sec_max: 0,
                latency_p99_ms_max: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_advise_applies_safety_margin_to_all_dimensions() {
        let a = DefaultThresholdAdvisor;
        let steps = vec![step(32, 104_300.0, 4.10)];
        let p = a.advise(&steps, 0, 70);
        // ceil(32 * 0.70) = ceil(22.4) = 23
        assert_eq!(p.requests_active_max, 23);
        // ceil(104300 * 0.70) = ceil(73010.0) = 73010
        assert_eq!(p.requests_per_sec_max, 73010);
        // 4.10 * 0.70 = 2.87
        assert!((p.latency_p99_ms_max - 2.87).abs() < 0.01);
    }

    #[test]
    fn test_fallback_policy_uses_last_step() {
        let a = DefaultThresholdAdvisor;
        let steps = vec![step(1, 1000.0, 1.0), step(2, 2000.0, 2.0)];
        let p = a.fallback_policy(&steps, 70);
        // last step: concurrency=2, rps=2000, p99=2.0
        assert_eq!(p.requests_active_max, 2); // ceil(2 * 0.70) = 2
        assert_eq!(p.requests_per_sec_max, 1400); // ceil(2000 * 0.70) = 1400
        assert!((p.latency_p99_ms_max - 1.40).abs() < 0.01);
    }

    #[test]
    fn test_advise_with_100_pct_margin_matches_knee_values() {
        let a = DefaultThresholdAdvisor;
        let steps = vec![step(16, 50_000.0, 5.0)];
        let p = a.advise(&steps, 0, 100);
        assert_eq!(p.requests_active_max, 16);
        assert_eq!(p.requests_per_sec_max, 50_000);
        assert!((p.latency_p99_ms_max - 5.0).abs() < 0.01);
    }
}
