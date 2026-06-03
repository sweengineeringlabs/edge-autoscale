//! Integration tests for all public `BenchFacade` methods.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain::Domain;
use swe_edge_autoscale_bench::{BenchConfig, BenchConfigBuilder, BenchFacade, StepResultBuilder};

fn fast_config() -> BenchConfig {
    BenchConfigBuilder::new()
        .with_concurrency_steps(vec![1, 2])
        .with_step_duration_secs(1)
        .with_warmup_secs(0)
        .build()
}

fn echo() -> Arc<dyn swe_edge_autoscale_bench::BenchHandler> {
    BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string())
}

/// @covers: BenchFacade::create_bench_runner
#[tokio::test(flavor = "multi_thread")]
async fn test_saf_create_bench_runner_produces_report() {
    let report = BenchFacade::create_bench_runner(fast_config())
        .run(echo())
        .await
        .unwrap();
    assert_eq!(report.steps.len(), 2);
}

/// @covers: BenchFacade::adapt_handler
#[tokio::test(flavor = "multi_thread")]
async fn test_saf_adapt_handler_calls_underlying_handler() {
    let handler = echo();
    let result = handler.call().await;
    assert!(
        result.is_ok(),
        "adapt_handler must produce a working handler"
    );
}

/// @covers: BenchFacade::validate_config
#[test]
fn test_saf_validate_config_passes_for_valid_config() {
    assert!(BenchFacade::validate_config(&BenchConfig::default()).is_ok());
}

/// @covers: BenchFacade::validate_config
#[test]
fn test_saf_validate_config_fails_for_empty_steps() {
    let bad = BenchConfig {
        concurrency_steps: vec![],
        ..Default::default()
    };
    assert!(BenchFacade::validate_config(&bad).is_err());
}

/// @covers: BenchFacade::run
#[tokio::test(flavor = "multi_thread")]
async fn test_saf_run_delegates_to_processor() {
    let report = BenchFacade::run(fast_config(), echo()).await.unwrap();
    assert!(!report.steps.is_empty());
}

/// @covers: StepResultBuilder::build
#[test]
fn test_saf_step_result_builder_roundtrip() {
    let step = StepResultBuilder::new()
        .with_concurrency(4)
        .with_rps(2000.0)
        .build();
    assert_eq!(step.concurrency, 4);
    assert!((step.rps - 2000.0).abs() < f64::EPSILON);
}

/// @covers: BenchRunner::with_knee_detector
#[tokio::test(flavor = "multi_thread")]
async fn test_saf_with_knee_detector_overrides_algorithm() {
    use swe_edge_autoscale_bench::{KneeDetector, StepResult};

    struct NeverKnee;
    impl KneeDetector for NeverKnee {
        fn detect(&self, _: &[StepResult]) -> Option<usize> {
            None
        }
    }

    let report = BenchFacade::create_bench_runner(fast_config())
        .with_knee_detector(Arc::new(NeverKnee))
        .run(echo())
        .await
        .unwrap();
    assert_eq!(report.knee_index, None);
}
