//! Interface contract for the plateau-based knee detector.
//!
//! The concrete implementation is [`crate::core::knee::PlateauDetector`].
//!
//! Knee = first concurrency step where RPS growth from the previous step
//! falls below `plateau_rps_growth_pct` percent.

/// A type alias for a boxed plateau-style [`KneeDetector`] implementation.
///
/// Use this to accept any plateau detector without naming the concrete type.
///
/// [`KneeDetector`]: crate::api::knee::knee_detector::KneeDetector
pub type PlateauDetector = Box<dyn crate::api::knee::knee_detector::KneeDetector>;
