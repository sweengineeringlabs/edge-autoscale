//! Public `BenchRunner` type — the entry point for running a saturation benchmark.

use std::sync::Arc;

use crate::api::advisor::traits::threshold_advisor::ThresholdAdvisor;
use crate::api::error::bench_error::BenchError;
use crate::api::knee::traits::knee_detector::KneeDetector;
use crate::api::traits::bench_handler::BenchHandler;
use crate::api::vo::bench_config::BenchConfig;
use crate::api::vo::report::Report;

use crate::api::step::traits::runner_factory::RunnerFactory;

/// Entry point for running a saturation benchmark.
///
/// Construct via [`crate::saf::BenchFacade::create_bench_runner`] which wires in the
/// default `Runner` and `ThresholdAdvisor` implementations.
pub struct BenchRunner {
    pub(crate) config: BenchConfig,
    pub(crate) knee_detector: Arc<dyn KneeDetector>,
    pub(crate) runner_factory: Arc<dyn RunnerFactory>,
    pub(crate) threshold_advisor: Arc<dyn ThresholdAdvisor>,
}

impl BenchRunner {
    /// Replace the knee detector with a custom implementation (SPI extension).
    ///
    /// ```rust,ignore
    /// BenchFacade::create_bench_runner(config)
    ///     .with_knee_detector(Arc::new(MyAlgo))
    ///     .run(handler)
    ///     .await
    /// ```
    pub fn with_knee_detector(mut self, detector: Arc<dyn KneeDetector>) -> Self {
        self.knee_detector = detector;
        self
    }

    /// Run the full benchmark and return a [`Report`].
    pub async fn run(&self, handler: Arc<dyn BenchHandler>) -> Result<Report, BenchError> {
        if self.config.concurrency_steps.is_empty() {
            return Err(BenchError::NoSteps);
        }

        let runner = self.runner_factory.create(
            handler,
            self.config.warmup_secs,
            self.config.step_duration_secs,
        );
        let mut steps = Vec::with_capacity(self.config.concurrency_steps.len());
        for &concurrency in &self.config.concurrency_steps {
            steps.push(runner.run_step(concurrency).await);
        }
        let knee_index = self.knee_detector.detect(&steps);
        let policy = match knee_index {
            Some(idx) => self
                .threshold_advisor
                .advise(&steps, idx, self.config.safety_margin_pct),
            None => self
                .threshold_advisor
                .fallback_policy(&steps, self.config.safety_margin_pct),
        };
        Ok(Report {
            steps,
            knee_index,
            policy,
        })
    }
}
