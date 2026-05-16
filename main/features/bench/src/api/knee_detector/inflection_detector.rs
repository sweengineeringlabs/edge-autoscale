//! Interface contract for the inflection knee detection algorithm.
//!
//! The concrete implementation is
//! [`crate::core::knee_detector::InflectionDetector`].
//!
//! Knee = first step where Δp99ms / ΔRPS_normalised exceeds
//! `inflection_delta_ratio` — the Little's Law saturation signal.

#[cfg(test)]
mod tests {
    use crate::api::knee_detector::KneeDetector;
    use crate::api::load_report::StepResult;

    fn step(concurrency: usize, rps: f64, p99_ms: f64) -> StepResult {
        StepResult { concurrency, rps, p50_ms: 0.0, p95_ms: 0.0, p99_ms, p99_9_ms: 0.0, error_count: 0 }
    }

    /// Minimal inline implementation of the inflection contract — verifies the
    /// documented invariant without depending on the concrete `InflectionDetector`.
    struct InlineInflection { delta_ratio: f64 }
    impl KneeDetector for InlineInflection {
        fn detect(&self, steps: &[StepResult]) -> Option<usize> {
            if steps.len() < 2 { return None; }
            let base = steps[0].rps;
            if base <= 0.0 { return None; }
            for i in 1..steps.len() {
                let d_p99  = steps[i].p99_ms - steps[i - 1].p99_ms;
                let d_rps  = (steps[i].rps - steps[i - 1].rps) / base;
                if d_rps <= 0.0 { return Some(i); }
                if d_p99 / d_rps > self.delta_ratio { return Some(i); }
            }
            None
        }
    }

    #[test]
    fn test_inflection_contract_latency_explosion_triggers_knee() {
        let d = InlineInflection { delta_ratio: 2.0 };
        let steps = vec![
            step(1, 1000.0, 0.5),
            step(2, 1900.0, 0.6),
            step(4, 2100.0, 5.0), // Δp99=4.4, ΔRPSnorm=0.2 → ratio=22 > 2.0
        ];
        assert_eq!(d.detect(&steps), Some(2));
    }

    #[test]
    fn test_inflection_contract_returns_none_when_latency_stable() {
        let d = InlineInflection { delta_ratio: 2.0 };
        let steps = vec![
            step(1, 1000.0, 1.0),
            step(2, 2000.0, 1.1),
            step(4, 3000.0, 1.2),
        ];
        assert_eq!(d.detect(&steps), None);
    }
}
