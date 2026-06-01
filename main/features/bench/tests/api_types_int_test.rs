//! Integration tests for api/ public types.

use swe_edge_autoscale_bench::{
    AutoscalePolicy, BenchConfig, BenchConfigBuilder, BenchError, KneeDetectionConfig, Report,
    StepResult, StepResultBuilder,
};

// --- BenchConfig ---

/// @covers: BenchConfig::default
#[test]
fn test_bench_config_default_has_non_empty_concurrency_steps() {
    let cfg = BenchConfig::default();
    assert!(!cfg.concurrency_steps.is_empty());
}

/// @covers: BenchConfig::default
#[test]
fn test_bench_config_default_safety_margin_pct_is_70() {
    assert_eq!(BenchConfig::default().safety_margin_pct, 70);
}

// --- KneeDetectionConfig ---

/// @covers: KneeDetectionConfig::default
#[test]
fn test_knee_detection_config_default_algorithm_is_inflection() {
    assert_eq!(KneeDetectionConfig::default().algorithm, "inflection");
}

/// @covers: KneeDetectionConfig::default
#[test]
fn test_knee_detection_config_default_plateau_threshold_is_5() {
    assert!((KneeDetectionConfig::default().plateau_rps_growth_pct - 5.0).abs() < f64::EPSILON);
}

// --- BenchError ---

/// @covers: BenchError::StepFailed
#[test]
fn test_bench_error_step_failed_display_contains_message() {
    let err = BenchError::StepFailed("timeout".into());
    assert!(err.to_string().contains("timeout"));
}

/// @covers: BenchError::NoSteps
#[test]
fn test_bench_error_no_steps_display_mentions_concurrency_steps() {
    let err = BenchError::NoSteps;
    assert!(err.to_string().contains("concurrency_steps"));
}

// --- StepResult ---

/// @covers: StepResultBuilder::build
#[test]
fn test_step_result_builder_sets_all_fields() {
    let s = StepResultBuilder::new()
        .with_concurrency(16)
        .with_rps(10_000.0)
        .with_p50_ms(0.5)
        .with_p95_ms(1.0)
        .with_p99_ms(2.0)
        .with_p99_9_ms(5.0)
        .with_error_count(7)
        .build();
    assert_eq!(s.concurrency, 16);
    assert_eq!(s.error_count, 7);
    assert!((s.rps - 10_000.0).abs() < f64::EPSILON);
}

// --- AutoscalePolicy ---

/// @covers: AutoscalePolicy
#[test]
fn test_autoscale_policy_fields_are_accessible() {
    let p = AutoscalePolicy {
        requests_active_max: 10,
        requests_per_sec_max: 5000,
        latency_p99_ms_max: 2.5,
    };
    assert_eq!(p.requests_active_max, 10);
    assert_eq!(p.requests_per_sec_max, 5000);
    assert!((p.latency_p99_ms_max - 2.5).abs() < f64::EPSILON);
}

// --- Report ---

/// @covers: Report::summary_table
#[test]
fn test_report_summary_table_contains_autoscale_section() {
    let report = make_report();
    let table = report.summary_table();
    assert!(table.contains("[autoscale]"));
}

/// @covers: Report::recommended_policy_toml
#[test]
fn test_report_recommended_policy_toml_has_all_keys() {
    let toml = make_report().recommended_policy_toml();
    assert!(toml.contains("requests_active_max"));
    assert!(toml.contains("requests_per_sec_max"));
    assert!(toml.contains("latency_p99_ms_max"));
}

/// @covers: Report::to_json
#[test]
fn test_report_to_json_is_valid_json_object() {
    let json = make_report().to_json();
    assert!(json.starts_with('{') && json.ends_with('}'));
    assert!(json.contains("\"policy\""));
    assert!(json.contains("\"steps\""));
}

/// @covers: Report
#[test]
fn test_report_has_accessible_steps() {
    let report: Report = make_report();
    assert_eq!(report.steps.len(), 1);
}

fn make_report() -> Report {
    Report {
        steps: vec![StepResult {
            concurrency: 1,
            rps: 1000.0,
            p50_ms: 0.5,
            p95_ms: 1.0,
            p99_ms: 2.0,
            p99_9_ms: 4.0,
            error_count: 0,
        }],
        knee_index: Some(0),
        policy: AutoscalePolicy {
            requests_active_max: 1,
            latency_p99_ms_max: 1.4,
            requests_per_sec_max: 700,
        },
    }
}

// --- BenchConfigBuilder ---

/// @covers: BenchConfigBuilder::build
#[test]
fn test_bench_config_builder_all_overrides() {
    let config = BenchConfigBuilder::new()
        .with_concurrency_steps(vec![1, 2, 4])
        .with_step_duration_secs(2)
        .with_warmup_secs(1)
        .with_safety_margin_pct(80)
        .build();
    assert_eq!(config.concurrency_steps, vec![1, 2, 4]);
    assert_eq!(config.step_duration_secs, 2);
    assert_eq!(config.warmup_secs, 1);
    assert_eq!(config.safety_margin_pct, 80);
}

/// @covers: BenchConfigBuilder::build
#[test]
fn test_bench_config_builder_defaults_match_bench_config_default() {
    let from_builder = BenchConfigBuilder::new().build();
    let direct = BenchConfig::default();
    assert_eq!(from_builder.concurrency_steps, direct.concurrency_steps);
    assert_eq!(from_builder.step_duration_secs, direct.step_duration_secs);
}
