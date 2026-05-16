//! Saturation knee detection trait.

pub(crate) mod inflection_detector;
pub(crate) mod plateau_detector;

use crate::api::load_report::StepResult;

/// Detects the concurrency step at which the handler saturates.
///
/// Returns `Some(index)` into the `steps` slice, or `None` if no knee
/// is found (e.g. the concurrency range is too narrow).
///
/// Implement this trait and wire it in via
/// [`crate::saf::BenchRunner::with_knee_detector`] to supply a custom
/// detection algorithm.
pub trait KneeDetector: Send + Sync {
    /// Inspect all completed step results and return the index of the knee step.
    fn detect(&self, steps: &[StepResult]) -> Option<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysKneeAtZero;
    impl KneeDetector for AlwaysKneeAtZero {
        fn detect(&self, steps: &[StepResult]) -> Option<usize> {
            if steps.is_empty() { None } else { Some(0) }
        }
    }

    struct NeverKnee;
    impl KneeDetector for NeverKnee {
        fn detect(&self, _steps: &[StepResult]) -> Option<usize> { None }
    }

    fn step(concurrency: usize) -> StepResult {
        StepResult { concurrency, rps: 100.0, p50_ms: 1.0, p95_ms: 2.0, p99_ms: 3.0, p99_9_ms: 4.0, error_count: 0 }
    }

    #[test]
    fn test_detect_returns_some_for_non_empty_steps() {
        let d = AlwaysKneeAtZero;
        assert_eq!(d.detect(&[step(1), step(2)]), Some(0));
    }

    #[test]
    fn test_detect_returns_none_for_empty_steps() {
        let d = AlwaysKneeAtZero;
        assert_eq!(d.detect(&[]), None);
    }

    #[test]
    fn test_detect_returns_none_when_impl_finds_no_knee() {
        let d = NeverKnee;
        assert_eq!(d.detect(&[step(1), step(2), step(4)]), None);
    }
}
