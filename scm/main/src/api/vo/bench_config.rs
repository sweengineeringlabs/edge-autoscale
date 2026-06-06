//! Top-level bench run configuration.

use serde::Deserialize;

use crate::api::knee::vo::knee_detection_config::KneeDetectionConfig;

/// Top-level bench configuration.
///
/// Loaded from the `[bench]` section of `application.toml` via
/// [`swe_edge_configbuilder::ConfigSection::load`]. All fields have
/// safe defaults; override only what you need.
///
/// # Examples
///
/// ```rust
/// use swe_edge_autoscale_bench::BenchConfig;
///
/// // SWE baseline: probe 1→128 concurrency with 10s steps.
/// let cfg = BenchConfig::default();
/// assert_eq!(cfg.concurrency_steps, vec![1, 2, 4, 8, 16, 32, 64, 128]);
/// assert_eq!(cfg.step_duration_secs, 10);
/// assert_eq!(cfg.warmup_secs, 2);
/// assert_eq!(cfg.safety_margin_pct, 70);
///
/// // Custom: fewer steps for a quick smoke test.
/// let cfg = BenchConfig {
///     concurrency_steps: vec![1, 4, 16],
///     step_duration_secs: 5,
///     ..BenchConfig::default()
/// };
/// assert_eq!(cfg.concurrency_steps.len(), 3);
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct BenchConfig {
    /// Concurrency levels to probe, in ascending order.
    #[serde(default = "BenchConfig::default_concurrency_steps")]
    pub concurrency_steps: Vec<usize>,

    /// Measurement window per concurrency step (seconds), after warmup.
    #[serde(default = "BenchConfig::default_step_duration_secs")]
    pub step_duration_secs: u64,

    /// Warm-up period at the start of each step (seconds). Results discarded.
    #[serde(default = "BenchConfig::default_warmup_secs")]
    pub warmup_secs: u64,

    /// Recommended threshold = knee_value × (safety_margin_pct / 100).
    #[serde(default = "BenchConfig::default_safety_margin_pct")]
    pub safety_margin_pct: u8,

    /// Knee detection sub-configuration.
    #[serde(default)]
    pub knee_detection: KneeDetectionConfig,
}

impl BenchConfig {
    fn default_concurrency_steps() -> Vec<usize> {
        vec![1, 2, 4, 8, 16, 32, 64, 128]
    }

    fn default_step_duration_secs() -> u64 {
        10
    }

    fn default_warmup_secs() -> u64 {
        2
    }

    fn default_safety_margin_pct() -> u8 {
        70
    }
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            concurrency_steps: Self::default_concurrency_steps(),
            step_duration_secs: Self::default_step_duration_secs(),
            warmup_secs: Self::default_warmup_secs(),
            safety_margin_pct: Self::default_safety_margin_pct(),
            knee_detection: KneeDetectionConfig::default(),
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for BenchConfig {
    fn section_name() -> &'static str {
        // TOML section key: `[bench]`
        const SECTION: &str = "bench";
        SECTION
    }
}
