//! Knee detection algorithm and sensitivity parameters.

use serde::Deserialize;

/// Knee detection algorithm and sensitivity parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct KneeDetectionConfig {
    /// `"plateau"` or `"inflection"`. Custom algorithms plug in via `spi/`.
    #[serde(default = "KneeDetectionConfig::default_algorithm")]
    pub algorithm: String,

    /// Plateau algorithm: RPS growth below this % between steps triggers knee.
    #[serde(default = "KneeDetectionConfig::default_plateau_rps_growth_pct")]
    pub plateau_rps_growth_pct: f64,

    /// Inflection algorithm: Δp99ms / ΔRPS_normalised above this triggers knee.
    #[serde(default = "KneeDetectionConfig::default_inflection_delta_ratio")]
    pub inflection_delta_ratio: f64,
}

impl KneeDetectionConfig {
    fn default_algorithm() -> String {
        "inflection".into()
    }

    fn default_plateau_rps_growth_pct() -> f64 {
        5.0
    }

    fn default_inflection_delta_ratio() -> f64 {
        2.0
    }
}

impl Default for KneeDetectionConfig {
    fn default() -> Self {
        Self {
            algorithm: Self::default_algorithm(),
            plateau_rps_growth_pct: Self::default_plateau_rps_growth_pct(),
            inflection_delta_ratio: Self::default_inflection_delta_ratio(),
        }
    }
}
