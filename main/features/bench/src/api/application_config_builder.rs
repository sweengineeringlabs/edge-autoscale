use std::sync::Arc;

use crate::api::bench_config::BenchConfig;
use crate::api::knee_detector::KneeDetector;

/// Builder for autoscale benchmark configuration with optional SPI extensions.
pub struct ApplicationConfigBuilder {
    /// Parsed benchmark configuration.
    pub(crate) config: BenchConfig,
    /// Optional custom knee detector (SPI extension).
    pub(crate) knee_detector: Option<Arc<dyn KneeDetector>>,
}
