//! Tests verifying the gateway egress integration.
//!
//! The egress gateway re-exports the full public surface from saf/.
//! These tests verify that all key types are accessible through the public API.

use swe_edge_autoscale_bench::{BenchConfig, BenchFacade, Report, StepResult};

/// @covers: gateway::egress
#[test]
fn test_gateway_egress_exposes_bench_config() {
    let _ = BenchConfig::default();
}

/// @covers: gateway::egress
#[test]
fn test_gateway_egress_exposes_report_type() {
    let report: Option<Report> = None;
    assert!(report.is_none());
}

/// @covers: gateway::egress
#[test]
fn test_gateway_egress_exposes_step_result() {
    let _step: Option<StepResult> = None;
}
