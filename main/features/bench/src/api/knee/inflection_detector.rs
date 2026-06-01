//! Interface contract for the inflection-based knee detector.
//!
//! The concrete implementation is [`crate::core::knee::InflectionDetector`].
//!
//! Knee = first step where Δp99ms / ΔRPS_normalised exceeds
//! `inflection_delta_ratio` — the Little's Law saturation signal.

/// A type alias for a boxed inflection-style [`KneeDetector`] implementation.
///
/// Use this to accept any inflection detector without naming the concrete type.
///
/// [`KneeDetector`]: crate::api::knee::knee_detector::KneeDetector
pub type InflectionDetector = Box<dyn crate::api::knee::knee_detector::KneeDetector>;
