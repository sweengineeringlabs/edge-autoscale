//! Trait for driving load against the system under test.

use futures::future::BoxFuture;

use crate::api::vo::step_result::StepResult;

/// Runs one concurrency step and returns the measured results.
///
/// The default backend is provided by the `spi/` layer.
pub trait Runner: Send + Sync {
    /// Drive `concurrency` concurrent tasks for the configured duration and
    /// return the aggregate latency histogram and throughput for this step.
    fn run_step(&self, concurrency: usize) -> BoxFuture<'_, StepResult>;
}
