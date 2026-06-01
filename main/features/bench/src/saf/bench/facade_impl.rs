//! `BenchFacade` method implementations.

use std::sync::Arc;

use crate::api::bench::bench_config::BenchConfig;
use crate::api::bench::bench_handler::BenchHandler;
use crate::api::knee::knee_detector::KneeDetector;
use crate::api::types::bench::bench_runner::BenchRunner;
use crate::api::types::bench_facade::BenchFacade;

impl BenchFacade {
    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    ///
    /// The config builder is built against SPI version [`crate::spi::SPI_VERSION`].
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let _spi_version = crate::spi::SPI_VERSION;
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Create a [`BenchRunner`] wired with the default `Runner` and `ThresholdAdvisor`.
    ///
    /// The knee detection algorithm is chosen from `config.knee_detection.algorithm`
    /// (`"plateau"` or `"inflection"`).
    pub fn create_bench_runner(config: BenchConfig) -> BenchRunner {
        use crate::core::advisor::DefaultThresholdAdvisor;
        use crate::core::knee::{InflectionDetector, PlateauDetector};
        use crate::core::step::DefaultRunnerFactory;

        let knee_detector: Arc<dyn KneeDetector> = match config.knee_detection.algorithm.as_str() {
            "plateau" => Arc::new(PlateauDetector {
                rps_growth_pct: config.knee_detection.plateau_rps_growth_pct,
            }),
            _ => Arc::new(InflectionDetector {
                delta_ratio: config.knee_detection.inflection_delta_ratio,
            }),
        };
        BenchRunner {
            config,
            knee_detector,
            runner_factory: Arc::new(DefaultRunnerFactory),
            threshold_advisor: Arc::new(DefaultThresholdAdvisor),
        }
    }

    /// Wrap an `edge-domain` [`edge_domain::Handler`] as a [`BenchHandler`].
    ///
    /// `make_request` is called once per handler invocation to produce the
    /// request value. Use a closure over your test fixtures:
    ///
    /// ```rust,ignore
    /// let target = BenchFacade::adapt_handler(
    ///     Domain::echo_handler("ping", "/ping"),
    ///     || "ping".to_string(),
    /// );
    /// ```
    pub fn adapt_handler<Req, Resp>(
        handler: Arc<dyn edge_domain::Handler<Req, Resp>>,
        make_request: impl Fn() -> Req + Send + Sync + 'static,
    ) -> Arc<dyn BenchHandler>
    where
        Req: Send + 'static,
        Resp: Send + 'static,
    {
        use crate::api::bench::bench_error::BenchError;
        use crate::api::bench::bench_future::BenchFuture;

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

    /// Validate a [`BenchConfig`] using the default validator.
    ///
    /// Returns `Ok(())` if the config is valid, or an error describing
    /// why the config is invalid.
    pub fn validate_config(
        config: &BenchConfig,
    ) -> Result<(), crate::api::bench::bench_error::BenchError> {
        use crate::api::traits::Validator as _;
        crate::core::validator::DefaultValidator.validate(config)
    }

    /// Run a complete bench and return the [`Report`].
    ///
    /// This method is a convenience wrapper that creates a [`DefaultProcessor`]
    /// internally and drives the bench run via the `Processor` trait.
    pub async fn run(
        config: BenchConfig,
        handler: Arc<dyn BenchHandler>,
    ) -> Result<crate::api::outcome::report::Report, crate::api::bench::bench_error::BenchError>
    {
        use crate::api::traits::Processor as _;
        let runner = Self::create_bench_runner(config);
        let processor = crate::core::processor::DefaultProcessor::new(runner, handler);
        processor.process().await
    }
}
