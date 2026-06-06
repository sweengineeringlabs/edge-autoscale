//! Builder for [`StepResult`].

use crate::api::vo::step_result::StepResult;

/// Fluent builder for [`StepResult`].
///
/// Useful in tests to construct step fixtures without spelling out every field.
#[derive(Debug, Default)]
pub struct StepResultBuilder {
    concurrency: Option<usize>,
    rps: Option<f64>,
    p50_ms: Option<f64>,
    p95_ms: Option<f64>,
    p99_ms: Option<f64>,
    p99_9_ms: Option<f64>,
    error_count: Option<u64>,
}

impl StepResultBuilder {
    /// Create a new builder with all fields at zero/default.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the concurrency level.
    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }

    /// Set the measured RPS.
    pub fn with_rps(mut self, rps: f64) -> Self {
        self.rps = Some(rps);
        self
    }

    /// Set p50 latency (ms).
    pub fn with_p50_ms(mut self, ms: f64) -> Self {
        self.p50_ms = Some(ms);
        self
    }

    /// Set p95 latency (ms).
    pub fn with_p95_ms(mut self, ms: f64) -> Self {
        self.p95_ms = Some(ms);
        self
    }

    /// Set p99 latency (ms).
    pub fn with_p99_ms(mut self, ms: f64) -> Self {
        self.p99_ms = Some(ms);
        self
    }

    /// Set p99.9 latency (ms).
    pub fn with_p99_9_ms(mut self, ms: f64) -> Self {
        self.p99_9_ms = Some(ms);
        self
    }

    /// Set the error count.
    pub fn with_error_count(mut self, count: u64) -> Self {
        self.error_count = Some(count);
        self
    }

    /// Build the [`StepResult`].
    pub fn build(self) -> StepResult {
        StepResult {
            concurrency: self.concurrency.unwrap_or(1),
            rps: self.rps.unwrap_or(0.0),
            p50_ms: self.p50_ms.unwrap_or(0.0),
            p95_ms: self.p95_ms.unwrap_or(0.0),
            p99_ms: self.p99_ms.unwrap_or(0.0),
            p99_9_ms: self.p99_9_ms.unwrap_or(0.0),
            error_count: self.error_count.unwrap_or(0),
        }
    }
}
