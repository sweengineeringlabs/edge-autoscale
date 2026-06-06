//! Extension hooks for downstream consumers.
//!
//! Downstream crates that depend on `swe-edge-autoscale-bench` can implement
//! [`crate::api::knee::traits::knee_detector::KneeDetector`] to supply a custom
//! saturation detection algorithm, then wire it in via
//! [`crate::api::types::bench_runner::BenchRunner::with_knee_detector`].
//!
//! ## Example
//!
//! ```rust,ignore
//! use swe_edge_autoscale_bench::{KneeDetector, StepResult, BenchFacade};
//!
//! struct MyKnee;
//! impl KneeDetector for MyKnee {
//!     fn detect(&self, steps: &[StepResult]) -> Option<usize> {
//!         steps.iter().position(|s| s.error_count > 0)
//!     }
//! }
//! ```

pub(crate) mod step;

/// Extension point version — bumped when the SPI contract changes.
pub(crate) const SPI_VERSION: u32 = 1;
