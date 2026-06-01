//! Integration tests for the `Processor` trait via `BenchFacade::run`.

use std::sync::Arc;

use edge_domain::Domain;
use swe_edge_autoscale_bench::{BenchConfig, BenchFacade};

/// @covers: BenchFacade::run
#[tokio::test(flavor = "multi_thread")]
async fn test_bench_facade_run_produces_report_for_echo_handler() {
    let config: BenchConfig = toml::from_str(
        "concurrency_steps = [1, 2]\nstep_duration_secs = 1\nwarmup_secs = 0\nsafety_margin_pct = 70",
    )
    .unwrap();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::run(config, handler).await.unwrap();
    assert_eq!(report.steps.len(), 2);
}

/// @covers: BenchFacade::run
#[tokio::test]
async fn test_bench_facade_run_returns_error_for_empty_concurrency_steps() {
    let config: BenchConfig = toml::from_str("concurrency_steps = []").unwrap();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let result = BenchFacade::run(config, handler).await;
    assert!(result.is_err(), "expected Err(NoSteps)");
}
