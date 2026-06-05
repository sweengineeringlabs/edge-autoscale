//! Trait for driving load against the system under test.

use futures::future::BoxFuture;

use crate::api::outcome::step_result::StepResult;

/// Runs one concurrency step and returns the measured results.
///
/// The concrete implementation is [`crate::core::step::TokioLoadRunner`].
pub trait Runner: Send + Sync {
    /// Drive `concurrency` concurrent tasks for the configured duration and
    /// return the aggregate latency histogram and throughput for this step.
    fn run_step(&self, concurrency: usize) -> BoxFuture<'_, StepResult>;
}
