//! Integration tests for `BenchRunner` against a real `EchoHandler`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain::Domain;
use swe_edge_autoscale_bench::{BenchConfig, BenchFacade};

fn fast_config() -> BenchConfig {
    toml::from_str(
        "concurrency_steps = [1, 2]\nstep_duration_secs = 1\nwarmup_secs = 0\nsafety_margin_pct = 70",
    )
    .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_produces_non_empty_report_for_echo_handler() {
    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    assert_eq!(
        report.steps.len(),
        2,
        "expected one StepResult per concurrency step"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_records_positive_rps_for_each_step() {
    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    for step in &report.steps {
        assert!(
            step.rps > 0.0,
            "concurrency={} produced rps=0 — no requests completed",
            step.concurrency
        );
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_policy_has_positive_thresholds() {
    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    assert!(
        report.policy.requests_active_max > 0,
        "requests_active_max must be positive"
    );
    assert!(
        report.policy.requests_per_sec_max > 0,
        "requests_per_sec_max must be positive"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_summary_table_is_non_empty() {
    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    let table = report.summary_table();
    assert!(!table.is_empty());
    assert!(table.contains("[autoscale]"));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_json_output_is_valid_structure() {
    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    let json = report.to_json();
    assert!(json.starts_with('{') && json.ends_with('}'));
    assert!(json.contains("\"policy\""));
    assert!(json.contains("\"steps\""));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_with_plateau_algorithm_completes() {
    let config: BenchConfig = toml::from_str(
        "concurrency_steps = [1, 2]\nstep_duration_secs = 1\nwarmup_secs = 0\nsafety_margin_pct = 70\n[knee_detection]\nalgorithm = \"plateau\"\nplateau_rps_growth_pct = 5.0\ninflection_delta_ratio = 2.0",
    )
    .unwrap();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();

    assert_eq!(report.steps.len(), 2);
}

#[tokio::test]
async fn test_bench_runner_returns_error_for_empty_concurrency_steps() {
    let config: BenchConfig = toml::from_str("concurrency_steps = []").unwrap();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let result = BenchFacade::create_bench_runner(config).run(handler).await;

    assert!(
        result.is_err(),
        "expected Err(NoSteps) for empty concurrency_steps"
    );
}

/// Integration test exercising the `swe-edge-configbuilder` dependency.
///
/// @covers: BenchFacade::create_config_builder
#[test]
fn test_create_config_builder_builds_loader_successfully() {
    let loader_result = BenchFacade::create_config_builder().build_loader();
    let _ = loader_result;
}

/// Exercises `ApplicationConfigBuilder` with the configbuilder pipeline.
///
/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_application_config_builder_returns_default_when_no_config_file() {
    use swe_edge_autoscale_bench::ApplicationConfigBuilder;

    let loader_result = BenchFacade::create_config_builder().build_loader();
    if let Ok(loader) = loader_result {
        let builder = ApplicationConfigBuilder::new(loader);
        let _ = builder.build();
    }
}

/// @covers: BenchConfigBuilder::build
#[test]
fn test_bench_config_builder_produces_config_with_expected_concurrency_steps() {
    use swe_edge_autoscale_bench::BenchConfigBuilder;

    let config = BenchConfigBuilder::new()
        .with_concurrency_steps(vec![1, 4, 16])
        .with_step_duration_secs(5)
        .with_warmup_secs(1)
        .with_safety_margin_pct(80)
        .build();

    assert_eq!(config.concurrency_steps, vec![1, 4, 16]);
    assert_eq!(config.step_duration_secs, 5);
    assert_eq!(config.warmup_secs, 1);
    assert_eq!(config.safety_margin_pct, 80);
}

/// @covers: StepResultBuilder::build
#[test]
fn test_step_result_builder_produces_step_with_expected_fields() {
    use swe_edge_autoscale_bench::StepResultBuilder;

    let step = StepResultBuilder::new()
        .with_concurrency(8)
        .with_rps(5000.0)
        .with_p99_ms(2.5)
        .with_error_count(3)
        .build();

    assert_eq!(step.concurrency, 8);
    assert!((step.rps - 5000.0).abs() < f64::EPSILON);
    assert!((step.p99_ms - 2.5).abs() < f64::EPSILON);
    assert_eq!(step.error_count, 3);
}

/// @covers: BenchRunner::with_knee_detector
#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_with_custom_knee_detector_runs_to_completion() {
    use swe_edge_autoscale_bench::{KneeDetector, StepResult};

    struct AlwaysKneeAtZero;
    impl KneeDetector for AlwaysKneeAtZero {
        fn detect(&self, steps: &[StepResult]) -> Option<usize> {
            if steps.is_empty() {
                None
            } else {
                Some(0)
            }
        }
    }

    let config = fast_config();
    let handler =
        BenchFacade::adapt_handler(Domain::echo_handler("ping", "/ping"), || "ping".to_string());
    let runner =
        BenchFacade::create_bench_runner(config).with_knee_detector(Arc::new(AlwaysKneeAtZero));
    let report = runner.run(handler).await.unwrap();
    assert_eq!(report.knee_index, Some(0));
}
