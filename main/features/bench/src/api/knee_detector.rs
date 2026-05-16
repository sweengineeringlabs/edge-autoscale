//! Saturation knee detection trait.

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
