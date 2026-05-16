//! Trait for driving load against the system under test.

use async_trait::async_trait;

use crate::api::load_report::StepResult;

/// Runs one concurrency step and returns the measured results.
///
/// The concrete implementation is [`crate::core::load_runner::TokioLoadRunner`].
#[async_trait]
pub trait LoadRunner: Send + Sync {
    /// Drive `concurrency` concurrent tasks for the configured duration and
    /// return the aggregate latency histogram and throughput for this step.
    async fn run_step(&self, concurrency: usize) -> StepResult;
}
