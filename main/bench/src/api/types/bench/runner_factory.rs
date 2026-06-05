//! Factory trait for creating `Runner` instances.

use std::sync::Arc;

use crate::api::bench::bench_handler::BenchHandler;
use crate::api::step::runner::Runner;

/// Factory that creates a [`Runner`] for a given handler.
///
/// Needed because `TokioLoadRunner` is constructed with the handler at
/// run time, not at `BenchRunner` construction time.
pub trait RunnerFactory: Send + Sync {
    /// Create a `Runner` for the given handler, warmup, and step duration.
    fn create(
        &self,
        handler: Arc<dyn BenchHandler>,
        warmup_secs: u64,
        step_duration_secs: u64,
    ) -> Arc<dyn Runner>;
}
