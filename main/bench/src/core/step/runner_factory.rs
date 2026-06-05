//! Default `RunnerFactory` implementation.

use std::sync::Arc;

use crate::api::bench::bench_handler::BenchHandler;
use crate::api::step::runner::Runner;
use crate::api::types::bench::runner_factory::RunnerFactory;

use super::tokio_load_runner::TokioLoadRunner;

/// Creates [`TokioLoadRunner`] instances for each bench step.
pub(crate) struct DefaultRunnerFactory;

impl RunnerFactory for DefaultRunnerFactory {
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
    use crate::api::bench::bench_future::BenchFuture;

    struct DefaultRunnerFactoryTestHandler;
    impl BenchHandler for DefaultRunnerFactoryTestHandler {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: create
    #[test]
    fn test_create_returns_runner_for_given_handler() {
        let factory = DefaultRunnerFactory;
        let _runner = factory.create(Arc::new(DefaultRunnerFactoryTestHandler), 0, 1);
    }
}
