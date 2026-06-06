//! Tokio-backed `RunnerFactory` implementation.

use std::sync::Arc;

use crate::api::step::traits::runner::Runner;
use crate::api::step::traits::runner_factory::RunnerFactory;
use crate::api::traits::bench_handler::BenchHandler;

use super::tokio_load_runner::TokioLoadRunner;

/// Creates [`TokioLoadRunner`] instances for each bench step.
pub(crate) struct TokioRunnerFactory;

impl RunnerFactory for TokioRunnerFactory {
    fn create(
        &self,
        handler: Arc<dyn BenchHandler>,
        warmup_secs: u64,
        step_duration_secs: u64,
    ) -> Arc<dyn Runner> {
        Arc::new(TokioLoadRunner {
            handler,
            warmup_secs,
            step_duration_secs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::bench_future::BenchFuture;

    struct TokioRunnerFactoryTestHandler;
    impl BenchHandler for TokioRunnerFactoryTestHandler {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: create
    #[test]
    fn test_create_returns_runner_for_given_handler() {
        let factory = TokioRunnerFactory;
        let _runner = factory.create(Arc::new(TokioRunnerFactoryTestHandler), 0, 1);
    }
}
