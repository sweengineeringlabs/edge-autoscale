//! Measurements for a single concurrency step.

/// Measurements for a single concurrency step.
///
/// Produced by the bench runner for each entry in `concurrency_steps`.
/// Used by [`Report`] to detect the knee and emit the autoscale policy.
/// Construct directly in tests; in production, produced by `BenchFacade::run`.
///
/// [`Report`]: crate::Report
///
/// # Examples
///
/// ```rust
/// use swe_edge_autoscale_bench::StepResult;
///
/// let step = StepResult {
///     concurrency: 8,
///     rps: 420.0,
///     p50_ms: 18.5,
///     p95_ms: 55.0,
///     p99_ms: 120.0,
///     p99_9_ms: 250.0,
///     error_count: 0,
/// };
///
/// assert_eq!(step.concurrency, 8);
/// assert!(step.p99_ms < step.p99_9_ms);
/// assert_eq!(step.error_count, 0);
/// ```
#[derive(Debug, Clone)]
pub struct StepResult {
    /// Number of concurrent tasks during this step.
    pub concurrency: usize,
    /// Requests per second over the measurement window.
    pub rps: f64,
    /// Median latency in milliseconds.
    pub p50_ms: f64,
    /// 95th-percentile latency in milliseconds.
    pub p95_ms: f64,
    /// 99th-percentile latency in milliseconds.
    pub p99_ms: f64,
    /// 99.9th-percentile latency in milliseconds.
    pub p99_9_ms: f64,
    /// Number of calls that returned an error during the measurement window.
    pub error_count: u64,
}
