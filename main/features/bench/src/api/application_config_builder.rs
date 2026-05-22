use std::sync::Arc;

use crate::api::bench_config::BenchConfig;
use crate::api::bench_error::BenchError;
use crate::api::knee_detector::KneeDetector;

/// Builder for autoscale benchmark configuration with optional SPI extensions.
pub struct ApplicationConfigBuilder {
    /// Parsed benchmark configuration.
    pub(crate) config: BenchConfig,
    /// Optional custom knee detector (SPI extension).
    pub(crate) knee_detector: Option<Arc<dyn KneeDetector>>,
}

impl ApplicationConfigBuilder {
    /// Parse config from a TOML string and return a new builder.
    pub fn with_config(toml_text: &str) -> Result<Self, BenchError> {
        Ok(Self {
            config: BenchConfig::from_config(toml_text)?,
            knee_detector: None,
        })
    }

    /// Override the knee detector (SPI extension — takes precedence over `algorithm` in config).
    pub fn with_knee_detector(mut self, detector: Arc<dyn KneeDetector>) -> Self {
        self.knee_detector = Some(detector);
        self
    }
}
