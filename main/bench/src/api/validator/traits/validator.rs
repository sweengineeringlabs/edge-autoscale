//! Validator trait for bench configuration.

use crate::api::error::bench_error::BenchError;
use crate::api::vo::bench_config::BenchConfig;

/// Validates a [`BenchConfig`] before a bench run is started.
///
/// Implement this trait to enforce custom pre-run constraints on the
/// bench configuration (e.g. minimum step count, maximum concurrency cap).
pub trait Validator: Send + Sync {
    /// Validate the given `config`.
    ///
    /// Returns `Ok(())` if the config is valid, or `Err(BenchError)` if
    /// a constraint is violated.
    fn validate(&self, config: &BenchConfig) -> Result<(), BenchError>;
}
