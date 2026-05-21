//! SAF layer — autoscale-bench public facade.

use std::sync::Arc;

use crate::core::knee_detector::{InflectionDetector, PlateauDetector};
use crate::core::load_runner::TokioLoadRunner;
use crate::core::threshold_advisor::DefaultThresholdAdvisor;

pub use crate::api::architecture_config_builder::ArchitectureConfigBuilder;
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
/// let config  = BenchConfig::swe_default().unwrap();
/// let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
/// let report  = BenchRunner::new(config).run(handler).await.unwrap();
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

/// Construct an [`ApplicationConfigBuilder`] from a TOML string.
///
/// ```rust,ignore
/// let report = swe_edge_autoscale_bench::builder(include_str!("config/application.toml"))?
///     .build()
///     .run(handler)
///     .await?;
/// ```
pub fn builder(toml_text: &str) -> Result<ApplicationConfigBuilder, BenchError> {
    ApplicationConfigBuilder::with_config(toml_text)
}

impl ApplicationConfigBuilder {
    /// Parse config from a TOML string and return a new builder.
    pub fn with_config(toml_text: &str) -> Result<Self, BenchError> {
        Ok(Self {
            config: BenchConfig::from_config(toml_text)?,
            knee_detector: None,
        })
    }

    /// Override the knee detector (SPI extension — takes precedence over `algorithm` in config).
    pub fn with_knee_detector(mut self, detector: Arc<dyn KneeDetector>) -> Self {
        self.knee_detector = Some(detector);
        self
    }

    /// Finalize and produce a configured [`BenchRunner`].
    pub fn build(self) -> BenchRunner {
        let runner = BenchRunner::new(self.config);
        match self.knee_detector {
            Some(d) => runner.with_knee_detector(d),
            None => runner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_config_parses_valid_toml() {
        let b = ApplicationConfigBuilder::with_config("safety_margin_pct = 80").unwrap();
        assert_eq!(b.config.safety_margin_pct, 80);
    }

    #[test]
    fn test_builder_with_config_returns_error_on_invalid_toml() {
        assert!(ApplicationConfigBuilder::with_config("[[bad").is_err());
    }

    #[test]
    fn test_builder_build_returns_bench_runner() {
        let runner = ApplicationConfigBuilder::with_config("").unwrap().build();
        // BenchRunner is not Clone — just verify construction doesn't panic.
        drop(runner);
    }

    #[test]
    fn test_builder_factory_fn_parses_config() {
        let b = builder("safety_margin_pct = 60").unwrap();
        assert_eq!(b.config.safety_margin_pct, 60);
    }
}
