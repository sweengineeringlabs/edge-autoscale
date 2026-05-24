//! Integration tests for `BenchRunner` against a real `echo_handler`.

use edge_domain::echo_handler;
use swe_edge_autoscale_bench::{adapt_handler, BenchConfig, BenchRunner};

fn fast_config() -> BenchConfig {
    toml::from_str(
        "concurrency_steps = [1, 2]\nstep_duration_secs = 1\nwarmup_secs = 0\nsafety_margin_pct = 70",
    )
    .unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_produces_non_empty_report_for_echo_handler() {
    let config = fast_config();
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

    assert_eq!(
        report.steps.len(),
        2,
        "expected one StepResult per concurrency step"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_records_positive_rps_for_each_step() {
    let config = fast_config();
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

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
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

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
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

    let table = report.summary_table();
    assert!(!table.is_empty());
    assert!(table.contains("[autoscale]"));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_bench_runner_json_output_is_valid_structure() {
    let config = fast_config();
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

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
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let report = BenchRunner::new(config).run(handler).await.unwrap();

    assert_eq!(report.steps.len(), 2);
}

#[tokio::test]
async fn test_bench_runner_returns_error_for_empty_concurrency_steps() {
    let config: BenchConfig = toml::from_str("concurrency_steps = []").unwrap();
    let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
    let result = BenchRunner::new(config).run(handler).await;

    assert!(
        result.is_err(),
        "expected Err(NoSteps) for empty concurrency_steps"
    );
}
