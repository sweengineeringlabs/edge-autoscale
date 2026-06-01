//! Interface contract for the runner factory.
//!
//! The concrete implementation is [`crate::core::step::DefaultRunnerFactory`].
//! The factory interface is [`crate::api::types::bench::runner_factory::RunnerFactory`].

/// A boxed trait object for step factories.
///
/// Used to store any [`RunnerFactory`] implementation by value.
///
/// [`RunnerFactory`]: crate::api::types::bench::runner_factory::RunnerFactory
pub type StepFactory = Box<dyn crate::api::types::bench::runner_factory::RunnerFactory>;
