//! Integration tests exercising the `swe-edge-configbuilder` dependency.
//!
//! These tests ensure that `BenchConfig` correctly integrates with the
//! configbuilder loading pipeline.

use swe_edge_autoscale_bench::{ApplicationConfigBuilder, BenchConfig, BenchFacade};
use swe_edge_configbuilder::ConfigSection as _;

/// @covers: BenchFacade::create_config_builder
#[test]
fn test_configbuilder_create_config_builder_returns_builder() {
    let builder = BenchFacade::create_config_builder();
    let _ = builder.build_loader();
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_configbuilder_application_config_builder_returns_result() {
    let loader_result = BenchFacade::create_config_builder().build_loader();
    if let Ok(loader) = loader_result {
        let _ = ApplicationConfigBuilder::new(loader).build();
    }
}

/// @covers: BenchConfig::section_name
#[test]
fn test_configbuilder_bench_config_section_name_is_bench() {
    assert_eq!(BenchConfig::section_name(), "bench");
}
