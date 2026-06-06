//! Integration tests for the `Validator` trait and `BenchFacade::validate_config`.

use swe_edge_autoscale_bench::{BenchConfig, BenchFacade};

/// @covers: BenchFacade::validate_config
#[test]
fn test_validate_config_returns_ok_for_default_config() {
    let config = BenchConfig::default();
    assert!(
        BenchFacade::validate_config(&config).is_ok(),
        "default BenchConfig must be valid"
    );
}

/// @covers: BenchFacade::validate_config
#[test]
fn test_validate_config_returns_err_for_empty_concurrency_steps() {
    let config = BenchConfig {
        concurrency_steps: vec![],
        ..Default::default()
    };
    assert!(
        BenchFacade::validate_config(&config).is_err(),
        "empty concurrency_steps must be invalid"
    );
}
