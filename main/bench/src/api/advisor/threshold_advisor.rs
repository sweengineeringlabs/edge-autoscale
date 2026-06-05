//! Trait for translating a knee step into autoscale thresholds.

use crate::api::outcome::{autoscale_policy::AutoscalePolicy, step_result::StepResult};

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
