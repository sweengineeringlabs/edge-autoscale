//! SAF layer — autoscale-bench public facade.

mod bench;

pub use crate::api::bench::bench_config::BenchConfig;
pub use crate::api::bench::bench_error::BenchError;
pub use crate::api::bench::bench_future::BenchFuture;
pub use crate::api::bench::bench_handler::BenchHandler;
pub use crate::api::bench::builder::BenchConfigBuilder;
pub use crate::api::bench::knee_detection_config::KneeDetectionConfig;
pub use crate::api::knee::inflection_detector::InflectionDetector;
pub use crate::api::knee::knee_detector::KneeDetector;
pub use crate::api::knee::plateau_detector::PlateauDetector;
pub use crate::api::outcome::autoscale_policy::AutoscalePolicy;
pub use crate::api::outcome::builder::StepResultBuilder;
pub use crate::api::outcome::report::Report;
pub use crate::api::outcome::step_result::StepResult;
pub use crate::api::step::runner_factory::RunnerFactory;
pub use crate::api::step::step_executor::StepExecutor;
pub use crate::api::step::step_factory::StepFactory;
pub use crate::api::types::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::types::bench_facade::BenchFacade;
