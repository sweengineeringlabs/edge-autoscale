//! TOML-deserializable configuration for a bench run.

use serde::Deserialize;

use crate::api::bench_error::BenchError;

/// Top-level bench configuration. Deserializes from a TOML string whose
/// content matches the `[bench]` section of `application.toml`.
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
            algorithm:              default_algorithm(),
            plateau_rps_growth_pct: default_plateau_rps_growth_pct(),
            inflection_delta_ratio: default_inflection_delta_ratio(),
        }
    }
}

fn default_concurrency_steps() -> Vec<usize>  { vec![1, 2, 4, 8, 16, 32, 64, 128] }
fn default_step_duration_secs() -> u64        { 10 }
fn default_warmup_secs()        -> u64        { 2 }
fn default_safety_margin_pct()  -> u8         { 70 }
fn default_algorithm()          -> String     { "inflection".into() }
fn default_plateau_rps_growth_pct() -> f64    { 5.0 }
fn default_inflection_delta_ratio() -> f64    { 2.0 }

impl BenchConfig {
    /// Parse from a TOML string containing the bench section content.
    pub fn from_config(toml_text: &str) -> Result<Self, BenchError> {
        toml::from_str(toml_text).map_err(|e| BenchError::ConfigParseFailed(e.to_string()))
    }

    /// Load the crate-shipped SWE defaults from `config/application.toml`.
    pub fn swe_default() -> Result<Self, BenchError> {
        Self::from_config(include_str!("../../../../config/application.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config_parses_minimal_toml() {
        let cfg = BenchConfig::from_config("").unwrap();
        assert_eq!(cfg.concurrency_steps, default_concurrency_steps());
        assert_eq!(cfg.step_duration_secs, default_step_duration_secs());
        assert_eq!(cfg.safety_margin_pct, default_safety_margin_pct());
    }

    #[test]
    fn test_from_config_overrides_concurrency_steps() {
        let cfg = BenchConfig::from_config("concurrency_steps = [1, 4, 16]").unwrap();
        assert_eq!(cfg.concurrency_steps, vec![1, 4, 16]);
    }

    #[test]
    fn test_from_config_overrides_knee_detection_algorithm() {
        let cfg = BenchConfig::from_config(
            "[knee_detection]\nalgorithm = \"plateau\"\nplateau_rps_growth_pct = 10.0\ninflection_delta_ratio = 3.0",
        )
        .unwrap();
        assert_eq!(cfg.knee_detection.algorithm, "plateau");
        assert!((cfg.knee_detection.plateau_rps_growth_pct - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_from_config_returns_error_on_invalid_toml() {
        let err = BenchConfig::from_config("concurrency_steps = [[[").unwrap_err();
        assert!(matches!(err, BenchError::ConfigParseFailed(_)));
    }

    #[test]
    fn test_swe_default_loads_without_error() {
        BenchConfig::swe_default().unwrap();
    }
}
