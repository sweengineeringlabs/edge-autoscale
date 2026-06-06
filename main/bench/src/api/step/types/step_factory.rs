//! Interface contract for the runner factory.
//!
//! The default backend is provided by the `spi/` layer.
//! The factory interface is [`crate::api::step::traits::runner_factory::RunnerFactory`].

/// A boxed trait object for step factories.
///
/// Used to store any [`RunnerFactory`] implementation by value.
///
/// [`RunnerFactory`]: crate::api::step::traits::runner_factory::RunnerFactory
pub type StepFactory = Box<dyn crate::api::step::traits::runner_factory::RunnerFactory>;
