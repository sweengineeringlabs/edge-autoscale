//! Inflection-based knee detection (Little's Law).

use crate::api::knee_detector::KneeDetector;
use crate::api::load_report::StepResult;

/// Knee = first step where Δp99ms / ΔRPS_normalised exceeds `delta_ratio`.
///
/// Detects the point where latency grows faster than throughput — the
/// mathematically precise saturation signal per Little's Law.
///
/// ΔRPS is normalised against the baseline (concurrency = 1) RPS so the
/// ratio is unit-free and stable across handlers with different throughput
/// magnitudes.
///
/// When RPS does not grow at all between two steps (ΔRPS ≤ 0), the knee is
/// declared at that step regardless of the ratio — the handler is already
/// saturated.
pub(crate) struct InflectionDetector {
    /// Knee threshold: Δp99ms / ΔRPS_normalised must exceed this value.
    pub(crate) delta_ratio: f64,
}

impl KneeDetector for InflectionDetector {
    fn detect(&self, steps: &[StepResult]) -> Option<usize> {
        if steps.len() < 2 {
            return None;
        }
        let base_rps = steps[0].rps;
        if base_rps <= 0.0 {
            return None;
        }
        for i in 1..steps.len() {
            let delta_p99      = steps[i].p99_ms - steps[i - 1].p99_ms;
            let delta_rps_norm = (steps[i].rps - steps[i - 1].rps) / base_rps;
            if delta_rps_norm <= 0.0 {
                return Some(i);
            }
            if delta_p99 / delta_rps_norm > self.delta_ratio {
                return Some(i);
            }
        }
        None
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
    fn test_detect_returns_none_when_fewer_than_two_steps() {
        let d = InflectionDetector { delta_ratio: 2.0 };
        assert_eq!(d.detect(&[step(1, 100.0, 1.0)]), None);
    }

    #[test]
    fn test_detect_returns_none_for_empty_steps() {
        let d = InflectionDetector { delta_ratio: 2.0 };
        assert_eq!(d.detect(&[]), None);
    }

    #[test]
    fn test_detect_returns_none_when_base_rps_is_zero() {
        let d = InflectionDetector { delta_ratio: 2.0 };
        let steps = vec![step(1, 0.0, 1.0), step(2, 100.0, 2.0)];
        assert_eq!(d.detect(&steps), None);
    }

    #[test]
    fn test_detect_identifies_latency_inflection_as_knee() {
        // step 0: concurrency=1,  rps=1000, p99=0.5ms → base_rps=1000
        // step 1: concurrency=2,  rps=1900, p99=0.6ms → delta_p99=0.1, delta_rps_norm=0.9 → ratio=0.11 — no knee
        // step 2: concurrency=4,  rps=2100, p99=5.0ms → delta_p99=4.4, delta_rps_norm=0.2 → ratio=22.0 > 2.0 ← knee
        let d = InflectionDetector { delta_ratio: 2.0 };
        let steps = vec![
            step(1, 1000.0, 0.5),
            step(2, 1900.0, 0.6),
            step(4, 2100.0, 5.0),
        ];
        assert_eq!(d.detect(&steps), Some(2));
    }

    #[test]
    fn test_detect_returns_knee_when_rps_stops_growing() {
        let d = InflectionDetector { delta_ratio: 2.0 };
        let steps = vec![
            step(1, 1000.0, 1.0),
            step(2, 1000.0, 2.0), // delta_rps_norm = 0 → implicit knee
        ];
        assert_eq!(d.detect(&steps), Some(1));
    }

    #[test]
    fn test_detect_returns_none_when_latency_stable_and_rps_growing() {
        let d = InflectionDetector { delta_ratio: 2.0 };
        let steps = vec![
            step(1, 1000.0, 1.0),
            step(2, 2000.0, 1.1),
            step(4, 3000.0, 1.2),
        ];
        assert_eq!(d.detect(&steps), None);
    }
}
