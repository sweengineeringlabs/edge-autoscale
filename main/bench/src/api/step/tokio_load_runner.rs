//! Interface contract counterpart for [`crate::core::step::tokio_load_runner`].
//!
//! The concrete implementation is [`crate::core::step::TokioLoadRunner`].

/// Marker trait for Tokio-based step execution units.
///
/// Implement this trait alongside [`crate::api::step::runner::Runner`] to
/// indicate that your implementation uses Tokio for concurrency.
/// The concrete implementation is [`crate::core::step::TokioLoadRunner`].
pub trait TokioLoadRunner: crate::api::step::runner::Runner {}
