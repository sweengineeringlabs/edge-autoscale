//! Plateau-based knee detection.

use crate::api::knee_detector::KneeDetector;
use crate::api::load_report::StepResult;

/// Knee = first step where RPS growth from the previous step falls below
/// `rps_growth_pct` percent.
///
/// Simple and reliable for handlers with clearly bounded throughput.
/// Requires at least 2 concurrency steps.
pub(crate) struct PlateauDetector {
    /// RPS growth threshold (%). Growth below this declares the knee.
    pub(crate) rps_growth_pct: f64,
}

impl KneeDetector for PlateauDetector {
    fn detect(&self, steps: &[StepResult]) -> Option<usize> {
        if steps.len() < 2 {
            return None;
        }
        for i in 1..steps.len() {
            let prev = steps[i - 1].rps;
            if prev <= 0.0 {
                continue;
            }
            let growth_pct = (steps[i].rps - prev) / prev * 100.0;
            if growth_pct < self.rps_growth_pct {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_detect_returns_none_when_fewer_than_two_steps() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        assert_eq!(d.detect(&[step(1, 100.0)]), None);
    }

    #[test]
    fn test_detect_returns_none_for_empty_steps() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        assert_eq!(d.detect(&[]), None);
    }

    #[test]
    fn test_detect_returns_knee_when_rps_growth_below_threshold() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        let steps = vec![
            step(1, 100.0),
            step(2, 190.0), // 90% growth — OK
            step(4, 193.0), // 1.6% growth — knee
        ];
        assert_eq!(d.detect(&steps), Some(2));
    }

    #[test]
    fn test_detect_returns_first_knee_not_subsequent_plateau() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        let steps = vec![
            step(1, 100.0),
            step(2, 190.0), // 90%
            step(4, 192.0), // 1.05% — first knee at index 2
            step(8, 192.5), // 0.26%
        ];
        assert_eq!(d.detect(&steps), Some(2));
    }

    #[test]
    fn test_detect_returns_none_when_rps_grows_consistently() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        let steps = vec![
            step(1, 100.0),
            step(2, 200.0),  // 100%
            step(4, 350.0),  // 75%
        ];
        assert_eq!(d.detect(&steps), None);
    }

    #[test]
    fn test_detect_skips_zero_rps_steps() {
        let d = PlateauDetector { rps_growth_pct: 5.0 };
        let steps = vec![
            step(1, 0.0),   // prev_rps = 0 → skipped
            step(2, 100.0),
            step(4, 101.0), // 1% growth — knee at index 2
        ];
        assert_eq!(d.detect(&steps), Some(2));
    }
}
