//! Bench run types: config, error, handler abstraction, and future type.

pub(crate) mod bench_config;
pub mod builder;
pub use builder::BenchConfigBuilder;
pub(crate) mod bench_error;
pub(crate) mod bench_future;
pub(crate) mod bench_handler;
pub(crate) mod knee_detection_config;
