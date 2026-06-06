//! Boxed step-execution alias.
//!
//! The default backend is provided by the `spi/` layer.
//! The primary runner interface is [`crate::api::step::traits::runner::Runner`].

/// A boxed trait object for step execution.
///
/// Used to store any [`Runner`] implementation by value.
///
/// [`Runner`]: crate::api::step::traits::runner::Runner
pub type StepExecutor = Box<dyn crate::api::step::traits::runner::Runner>;
