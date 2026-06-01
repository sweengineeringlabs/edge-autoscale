//! Error type for swe-edge-autoscale-bench.

/// Errors produced by the bench crate.
#[derive(Debug, thiserror::Error)]
pub enum BenchError {
    /// A single handler call returned an error during a measurement step.
    #[error("swe_edge_autoscale_bench: handler call failed — {0}")]
    StepFailed(String),

    /// `concurrency_steps` is empty; at least one step is required.
    #[error("swe_edge_autoscale_bench: concurrency_steps must contain at least one entry")]
    NoSteps,
}
