//! Interface contract for the Tokio-based load runner.
//!
//! The concrete implementation is [`crate::core::step::TokioLoadRunner`].
//! The primary runner interface is [`crate::api::step::runner::Runner`].

/// A boxed trait object for step execution.
///
/// Used to store any [`Runner`] implementation by value.
///
/// [`Runner`]: crate::api::step::runner::Runner
pub type StepExecutor = Box<dyn crate::api::step::runner::Runner>;
