//! TOML-deserializable configuration for a bench run.

use serde::Deserialize;

/// Top-level bench configuration.
///
/// Loaded from the `[bench]` section of `application.toml` via
/// [`swe_edge_configbuilder::ConfigSection::load`].
#[derive(Debug, Clone, Deserialize)]
pub struct BenchConfig {
    /// Concurrency levels to probe, in ascending order.
    #[serde(default = "default_concurrency_steps")]
    pub concurrency_steps: Vec<usize>,

    /// Measurement window per concurrency step (seconds), after warmup.
    #[serde(default = "default_step_duration_secs")]
    pub step_duration_secs: u64,

    /// Warm-up period at the start of each step (seconds). Results discarded.
    #[serde(default = "default_warmup_secs")]
    pub warmup_secs: u64,

    /// Recommended threshold = knee_value × (safety_margin_pct / 100).
    #[serde(default = "default_safety_margin_pct")]
    pub safety_margin_pct: u8,

    /// Knee detection sub-configuration.
    #[serde(default)]
    pub knee_detection: KneeDetectionConfig,
}

impl Default for BenchConfig {
    fn default() -> Self {
        Self {
            concurrency_steps: default_concurrency_steps(),
            step_duration_secs: default_step_duration_secs(),
            warmup_secs: default_warmup_secs(),
            safety_margin_pct: default_safety_margin_pct(),
            knee_detection: KneeDetectionConfig::default(),
        }
    }
}

impl swe_edge_configbuilder::ConfigSection for BenchConfig {
    fn section_name() -> &'static str {
        "bench"
    }
}

/// Knee detection algorithm and sensitivity parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct KneeDetectionConfig {
    /// `"plateau"` or `"inflection"`. Custom algorithms plug in via `spi/`.
    #[serde(default = "default_algorithm")]
    pub algorithm: String,

    /// Plateau algorithm: RPS growth below this % between steps triggers knee.
    #[serde(default = "default_plateau_rps_growth_pct")]
    pub plateau_rps_growth_pct: f64,

    /// Inflection algorithm: Δp99ms / ΔRPS_normalised above this triggers knee.
    #[serde(default = "default_inflection_delta_ratio")]
    pub inflection_delta_ratio: f64,
}

impl Default for KneeDetectionConfig {
    fn default() -> Self {
        Self {
            algorithm: default_algorithm(),
            plateau_rps_growth_pct: default_plateau_rps_growth_pct(),
            inflection_delta_ratio: default_inflection_delta_ratio(),
        }
    }
}

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
fn default_algorithm() -> String {
    "inflection".into()
}
fn default_plateau_rps_growth_pct() -> f64 {
    5.0
}
fn default_inflection_delta_ratio() -> f64 {
    2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_configbuilder::ConfigSection as _;

    /// @covers: BenchConfig::default
    #[test]
    fn test_bench_config_default_uses_expected_concurrency_steps() {
        assert_eq!(
            BenchConfig::default().concurrency_steps,
            vec![1, 2, 4, 8, 16, 32, 64, 128]
        );
    }

    /// @covers: BenchConfig::default
    #[test]
    fn test_bench_config_default_safety_margin_pct_is_70() {
        assert_eq!(BenchConfig::default().safety_margin_pct, 70);
    }

    /// @covers: ConfigSection::section_name
    #[test]
    fn test_bench_config_section_name_is_bench() {
        assert_eq!(BenchConfig::section_name(), "bench");
    }

    /// @covers: BenchConfig serde
    #[test]
    fn test_bench_config_deserializes_safety_margin_override() {
        let cfg: BenchConfig = toml::from_str("safety_margin_pct = 80").unwrap();
        assert_eq!(cfg.safety_margin_pct, 80);
    }

    /// @covers: BenchConfig serde
    #[test]
    fn test_bench_config_deserializes_concurrency_steps_override() {
        let cfg: BenchConfig = toml::from_str("concurrency_steps = [1, 4, 16]").unwrap();
        assert_eq!(cfg.concurrency_steps, vec![1, 4, 16]);
    }

    /// @covers: BenchConfig serde
    #[test]
    fn test_bench_config_deserializes_knee_detection_section() {
        let cfg: BenchConfig = toml::from_str(
            "[knee_detection]\nalgorithm = \"plateau\"\nplateau_rps_growth_pct = 10.0\ninflection_delta_ratio = 3.0",
        )
        .unwrap();
        assert_eq!(cfg.knee_detection.algorithm, "plateau");
        assert!((cfg.knee_detection.plateau_rps_growth_pct - 10.0).abs() < f64::EPSILON);
    }

    /// @covers: BenchConfig serde
    #[test]
    fn test_bench_config_rejects_invalid_toml() {
        assert!(toml::from_str::<BenchConfig>("concurrency_steps = [[[").is_err());
    }
}
