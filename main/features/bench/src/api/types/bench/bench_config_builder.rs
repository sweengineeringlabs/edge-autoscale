//! Builder for [`BenchConfig`].

use crate::api::bench::bench_config::BenchConfig;
use crate::api::bench::knee_detection_config::KneeDetectionConfig;

/// Fluent builder for [`BenchConfig`].
///
/// All fields default to the same values as [`BenchConfig::default`].
/// Use this builder in tests and benchmark harnesses to override individual
/// fields without constructing the full struct literal.
#[derive(Debug, Default)]
pub struct BenchConfigBuilder {
    concurrency_steps: Option<Vec<usize>>,
    step_duration_secs: Option<u64>,
    warmup_secs: Option<u64>,
    safety_margin_pct: Option<u8>,
    knee_detection: Option<KneeDetectionConfig>,
}

impl BenchConfigBuilder {
    /// Create a new builder pre-seeded with [`BenchConfig::default`] values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Override the concurrency steps to probe.
    pub fn with_concurrency_steps(mut self, steps: Vec<usize>) -> Self {
        self.concurrency_steps = Some(steps);
        self
    }

    /// Override the measurement window per concurrency step (seconds).
    pub fn with_step_duration_secs(mut self, secs: u64) -> Self {
        self.step_duration_secs = Some(secs);
        self
    }

    /// Override the warm-up period (seconds).
    pub fn with_warmup_secs(mut self, secs: u64) -> Self {
        self.warmup_secs = Some(secs);
        self
    }

    /// Override the safety margin percentage.
    pub fn with_safety_margin_pct(mut self, pct: u8) -> Self {
        self.safety_margin_pct = Some(pct);
        self
    }

    /// Override the knee detection configuration.
    pub fn with_knee_detection(mut self, cfg: KneeDetectionConfig) -> Self {
        self.knee_detection = Some(cfg);
        self
    }

    /// Build the final [`BenchConfig`], filling unset fields from defaults.
    pub fn build(self) -> BenchConfig {
        let def = BenchConfig::default();
        BenchConfig {
            concurrency_steps: self.concurrency_steps.unwrap_or(def.concurrency_steps),
            step_duration_secs: self.step_duration_secs.unwrap_or(def.step_duration_secs),
            warmup_secs: self.warmup_secs.unwrap_or(def.warmup_secs),
            safety_margin_pct: self.safety_margin_pct.unwrap_or(def.safety_margin_pct),
            knee_detection: self.knee_detection.unwrap_or(def.knee_detection),
        }
    }
}
