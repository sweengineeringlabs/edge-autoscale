//! SAF layer — autoscale-bench public facade.

use std::sync::Arc;

use crate::core::knee_detector::{InflectionDetector, PlateauDetector};
use crate::core::load_runner::TokioLoadRunner;
use crate::core::threshold_advisor::DefaultThresholdAdvisor;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
        .with_name(env!("CARGO_PKG_NAME"))
        .with_version(env!("CARGO_PKG_VERSION"))
}

pub use crate::api::bench_config::{BenchConfig, KneeDetectionConfig};
pub use crate::api::bench_error::BenchError;
pub use crate::api::bench_handler::{BenchFuture, BenchHandler};
pub use crate::api::knee_detector::KneeDetector;
pub use crate::api::load_report::{AutoscalePolicy, LoadReport, StepResult};
pub use crate::api::load_runner::LoadRunner;
pub use crate::api::threshold_advisor::ThresholdAdvisor;

/// Entry point for running a saturation benchmark.
///
/// ```rust,ignore
/// use swe_edge_configbuilder::ConfigSection as _;
/// let loader = swe_edge_autoscale_bench::create_config_builder().build_loader();
/// let config = swe_edge_autoscale_bench::BenchConfig::load(&loader).unwrap();
/// let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
/// let report = BenchRunner::new(config).run(handler).await.unwrap();
/// println!("{}", report.summary_table());
/// ```
pub struct BenchRunner {
    config: BenchConfig,
    knee_detector: Arc<dyn KneeDetector>,
}

impl BenchRunner {
    /// Construct a `BenchRunner`, selecting the knee detection algorithm from
    /// `config.knee_detection.algorithm` (`"plateau"` or `"inflection"`).
    pub fn new(config: BenchConfig) -> Self {
        let knee_detector: Arc<dyn KneeDetector> = match config.knee_detection.algorithm.as_str() {
            "plateau" => Arc::new(PlateauDetector {
                rps_growth_pct: config.knee_detection.plateau_rps_growth_pct,
            }),
            _ => Arc::new(InflectionDetector {
                delta_ratio: config.knee_detection.inflection_delta_ratio,
            }),
        };
        Self {
            config,
            knee_detector,
        }
    }

    /// Replace the knee detector with a custom implementation (SPI extension).
    ///
    /// ```rust,ignore
    /// BenchRunner::new(config).with_knee_detector(Arc::new(MyAlgo)).run(handler).await
    /// ```
    pub fn with_knee_detector(mut self, detector: Arc<dyn KneeDetector>) -> Self {
        self.knee_detector = detector;
        self
    }

    /// Run the full benchmark and return a [`LoadReport`].
    pub async fn run(&self, handler: Arc<dyn BenchHandler>) -> Result<LoadReport, BenchError> {
        if self.config.concurrency_steps.is_empty() {
            return Err(BenchError::NoSteps);
        }
        let runner = TokioLoadRunner {
            handler,
            warmup_secs: self.config.warmup_secs,
            step_duration_secs: self.config.step_duration_secs,
        };
        let mut steps = Vec::with_capacity(self.config.concurrency_steps.len());
        for &concurrency in &self.config.concurrency_steps {
            steps.push(runner.run_step(concurrency).await);
        }
        let knee_index = self.knee_detector.detect(&steps);
        let advisor = DefaultThresholdAdvisor;
        let policy = match knee_index {
            Some(idx) => advisor.advise(&steps, idx, self.config.safety_margin_pct),
            None => advisor.fallback_policy(&steps, self.config.safety_margin_pct),
        };
        Ok(LoadReport {
            steps,
            knee_index,
            policy,
        })
    }
}

/// Wrap an `edge-domain` [`edge_domain::Handler`] as a [`BenchHandler`].
///
/// `make_request` is called once per handler invocation to produce the
/// request value. Use a closure over your test fixtures:
///
/// ```rust,ignore
/// let target = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
/// ```
pub fn adapt_handler<Req, Resp>(
    handler: Arc<dyn edge_domain::Handler<Req, Resp>>,
    make_request: impl Fn() -> Req + Send + Sync + 'static,
) -> Arc<dyn BenchHandler>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    struct Adapter<Req, Resp> {
        handler: Arc<dyn edge_domain::Handler<Req, Resp>>,
        make_request: Box<dyn Fn() -> Req + Send + Sync>,
    }

    impl<Req, Resp> BenchHandler for Adapter<Req, Resp>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        fn call(&self) -> BenchFuture<'_> {
            let req = (self.make_request)();
            let handler = Arc::clone(&self.handler);
            Box::pin(async move {
                handler
                    .execute(req)
                    .await
                    .map(|_| ())
                    .map_err(|e| BenchError::StepFailed(e.to_string()))
            })
        }
    }

    Arc::new(Adapter {
        handler,
        make_request: Box::new(make_request),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_configbuilder::ConfigSection as _;

    /// @covers: BenchConfig::section_name
    #[test]
    fn test_bench_config_section_name_is_bench() {
        assert_eq!(BenchConfig::section_name(), "bench");
    }

    /// @covers: BenchRunner::new
    #[test]
    fn test_bench_runner_new_constructs_from_default_config() {
        let _runner = BenchRunner::new(BenchConfig::default());
    }

    /// @covers: BenchRunner::with_knee_detector
    #[test]
    fn test_bench_runner_with_knee_detector_replaces_detector() {
        use crate::api::load_report::StepResult;
        struct AlwaysNone;
        impl KneeDetector for AlwaysNone {
            fn detect(&self, _steps: &[StepResult]) -> Option<usize> {
                None
            }
        }
        let _runner =
            BenchRunner::new(BenchConfig::default()).with_knee_detector(Arc::new(AlwaysNone));
    }

    /// @covers: create_config_builder
    #[test]
    fn test_create_config_builder_builds_loader() {
        let _loader = create_config_builder().build_loader();
    }
}
