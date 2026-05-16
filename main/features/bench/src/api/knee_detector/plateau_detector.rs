//! Interface contract for the plateau knee detection algorithm.
//!
//! The concrete implementation is
//! [`crate::core::knee_detector::PlateauDetector`].
//!
//! Knee = first concurrency step where RPS growth from the previous step
//! falls below `plateau_rps_growth_pct` percent.

#[cfg(test)]
mod tests {
    use crate::api::knee_detector::KneeDetector;
    use crate::api::load_report::StepResult;

    fn step(concurrency: usize, rps: f64) -> StepResult {
        StepResult {
            concurrency,
            rps,
            p50_ms: 0.0,
            p95_ms: 0.0,
            p99_ms: 0.0,
            p99_9_ms: 0.0,
            error_count: 0,
        }
    }

    /// Minimal inline implementation of the plateau contract — verifies the
    /// documented invariant without depending on the concrete `PlateauDetector`.
    struct InlinePlateau {
        threshold_pct: f64,
    }
    impl KneeDetector for InlinePlateau {
        fn detect(&self, steps: &[StepResult]) -> Option<usize> {
            for i in 1..steps.len() {
                let prev = steps[i - 1].rps;
                if prev <= 0.0 {
                    continue;
                }
                if (steps[i].rps - prev) / prev * 100.0 < self.threshold_pct {
                    return Some(i);
                }
            }
            None
        }
    }

    #[test]
    fn test_plateau_contract_stalled_rps_triggers_knee() {
        let d = InlinePlateau { threshold_pct: 5.0 };
        let steps = vec![step(1, 100.0), step(2, 190.0), step(4, 192.0)];
        assert_eq!(d.detect(&steps), Some(2));
    }

    #[test]
    fn test_plateau_contract_returns_none_when_rps_grows_consistently() {
        let d = InlinePlateau { threshold_pct: 5.0 };
        let steps = vec![step(1, 100.0), step(2, 200.0), step(4, 380.0)];
        assert_eq!(d.detect(&steps), None);
    }
}
