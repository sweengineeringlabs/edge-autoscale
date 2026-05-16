//! Extension hooks for downstream consumers.
//!
//! Implement [`KneeDetector`] to supply a custom saturation detection
//! algorithm and wire it in via [`BenchRunner::with_knee_detector`].

pub use crate::api::knee_detector::KneeDetector;
pub use crate::api::load_report::StepResult;
