//! Measurements for a single concurrency step.

/// Measurements for a single concurrency step.
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
