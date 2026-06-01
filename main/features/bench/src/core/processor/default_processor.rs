//! Default `Processor` implementation for the bench crate.

use std::sync::Arc;

use crate::api::bench::bench_error::BenchError;
use crate::api::bench::bench_handler::BenchHandler;
use crate::api::outcome::report::Report;
use crate::api::traits::Processor;
use crate::api::types::bench::bench_runner::BenchRunner;

/// Drives a full bench run against a wired-in [`BenchHandler`].
///
/// This is the primary `Processor` implementation: it holds the [`BenchRunner`]
/// pre-configured with a handler and invokes it when [`Processor::process`] is called.
pub(crate) struct DefaultProcessor {
    runner: BenchRunner,
    handler: Arc<dyn BenchHandler>,
}

impl DefaultProcessor {
    /// Create a new processor from a runner and handler.
    pub(crate) fn new(runner: BenchRunner, handler: Arc<dyn BenchHandler>) -> Self {
        Self { runner, handler }
    }
}

impl Processor for DefaultProcessor {
    fn process(&self) -> futures::future::BoxFuture<'_, Result<Report, BenchError>> {
        let handler = Arc::clone(&self.handler);
        Box::pin(self.runner.run(handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::bench::bench_config::BenchConfig;
    use crate::api::bench::bench_future::BenchFuture;
    use crate::api::types::bench::bench_config_builder::BenchConfigBuilder;

    struct DefaultProcessorTestHandler;
    impl BenchHandler for DefaultProcessorTestHandler {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: new
    #[test]
    fn test_new_creates_processor_without_panicking() {
        use crate::saf::BenchFacade;
        let runner = BenchFacade::create_bench_runner(BenchConfig::default());
        let _processor = DefaultProcessor::new(runner, Arc::new(DefaultProcessorTestHandler));
    }

    #[tokio::test]
    async fn test_default_processor_process_returns_report() {
        use crate::saf::BenchFacade;

        let runner = BenchFacade::create_bench_runner(
            BenchConfigBuilder::new()
                .with_concurrency_steps(vec![1])
                .with_step_duration_secs(1)
                .with_warmup_secs(0)
                .build(),
        );
        let processor = DefaultProcessor::new(runner, Arc::new(DefaultProcessorTestHandler));
        let result = processor.process().await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[tokio::test]
    async fn test_default_processor_process_returns_error_for_empty_steps() {
        use crate::saf::BenchFacade;

        let runner = BenchFacade::create_bench_runner(BenchConfig {
            concurrency_steps: vec![],
            ..Default::default()
        });
        let processor = DefaultProcessor::new(runner, Arc::new(DefaultProcessorTestHandler));
        let result = processor.process().await;
        assert!(
            result.is_err(),
            "expected Err(NoSteps) for empty concurrency_steps"
        );
    }
}
