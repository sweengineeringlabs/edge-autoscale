//! Default bench configuration validator.

use crate::api::error::bench_error::BenchError;
use crate::api::validator::traits::Validator;
use crate::api::vo::bench_config::BenchConfig;

/// Validates that a [`BenchConfig`] meets minimum viability requirements.
///
/// Enforces:
/// - `concurrency_steps` must contain at least one entry.
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self, config: &BenchConfig) -> Result<(), BenchError> {
        if config.concurrency_steps.is_empty() {
            return Err(BenchError::NoSteps);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_non_empty_steps() {
        let v = DefaultValidator;
        let config = BenchConfig {
            concurrency_steps: vec![1, 2],
            ..Default::default()
        };
        assert!(v.validate(&config).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_error_for_empty_steps() {
        let v = DefaultValidator;
        let config = BenchConfig {
            concurrency_steps: vec![],
            ..Default::default()
        };
        assert!(v.validate(&config).is_err());
    }
}
