//! Integration tests for the `KneeDetector` trait and algorithm implementations.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_autoscale_bench::{BenchConfig, BenchFacade, KneeDetector, StepResult};

struct AlwaysKneeAtFirst;
impl KneeDetector for AlwaysKneeAtFirst {
    fn detect(&self, steps: &[StepResult]) -> Option<usize> {
        if steps.is_empty() {
            None
        } else {
            Some(0)
        }
    }
}

struct NeverKnee;
impl KneeDetector for NeverKnee {
    fn detect(&self, _: &[StepResult]) -> Option<usize> {
        None
    }
}

/// @covers: KneeDetector::detect
#[test]
fn test_always_knee_at_first_returns_some_zero_for_non_empty_steps() {
    let d = AlwaysKneeAtFirst;
    let steps = make_steps(3);
    assert_eq!(d.detect(&steps), Some(0));
}

/// @covers: KneeDetector::detect
#[test]
fn test_always_knee_at_first_returns_none_for_empty_steps() {
    let d = AlwaysKneeAtFirst;
    assert_eq!(d.detect(&[]), None);
}

/// @covers: KneeDetector::detect
#[test]
fn test_never_knee_always_returns_none() {
    let d = NeverKnee;
    assert_eq!(d.detect(&make_steps(5)), None);
}

/// @covers: BenchRunner::with_knee_detector
#[tokio::test(flavor = "multi_thread")]
async fn test_inflection_algorithm_runs_through_full_bench_cycle() {
    use edge_domain::Domain;
    let config: BenchConfig = toml::from_str(
        "concurrency_steps = [1]\nstep_duration_secs = 1\nwarmup_secs = 0\n[knee_detection]\nalgorithm = \"inflection\""
    ).unwrap();
    let handler = BenchFacade::adapt_handler(Domain::echo_handler("p", "/p"), || "p".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();
    assert_eq!(report.steps.len(), 1);
}

/// @covers: BenchRunner::with_knee_detector
#[tokio::test(flavor = "multi_thread")]
async fn test_plateau_algorithm_runs_through_full_bench_cycle() {
    use edge_domain::Domain;
    let config: BenchConfig = toml::from_str(
        "concurrency_steps = [1]\nstep_duration_secs = 1\nwarmup_secs = 0\n[knee_detection]\nalgorithm = \"plateau\""
    ).unwrap();
    let handler = BenchFacade::adapt_handler(Domain::echo_handler("p", "/p"), || "p".to_string());
    let report = BenchFacade::create_bench_runner(config)
        .run(handler)
        .await
        .unwrap();
    assert_eq!(report.steps.len(), 1);
}

fn make_steps(n: usize) -> Vec<StepResult> {
    (0..n)
        .map(|i| StepResult {
            concurrency: i + 1,
            rps: 1000.0 * (i + 1) as f64,
            p50_ms: 0.5,
            p95_ms: 1.0,
            p99_ms: 2.0,
            p99_9_ms: 4.0,
            error_count: 0,
        })
        .collect()
}
