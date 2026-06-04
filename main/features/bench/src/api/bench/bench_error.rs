//! Error type for swe-edge-autoscale-bench.

/// Errors produced by the bench crate.
///
/// `StepFailed` wraps handler errors from a single concurrency step — the
/// run aborts and reports which step failed. `NoSteps` is a configuration
/// error caught before any I/O begins.
///
/// # Examples
///
/// ```rust
/// use swe_edge_autoscale_bench::BenchError;
///
/// let err = BenchError::NoSteps;
/// assert!(err.to_string().contains("concurrency_steps"));
///
/// let err = BenchError::StepFailed("timeout after 30s".to_string());
/// assert!(err.to_string().contains("handler call failed"));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum BenchError {
    /// A single handler call returned an error during a measurement step.
    #[error("swe_edge_autoscale_bench: handler call failed — {0}")]
    StepFailed(String),

    /// `concurrency_steps` is empty; at least one step is required.
    #[error("swe_edge_autoscale_bench: concurrency_steps must contain at least one entry")]
    NoSteps,
}
