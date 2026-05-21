//! Builder for workspace architectural and application configuration.
///
/// Reads project metadata from `main/config/architecture.toml` at build time.
/// Not typically constructed directly — provided here to satisfy the
/// `<name>.toml → <Name>ConfigBuilder` convention for `architecture.toml`.
pub struct ArchitectureConfigBuilder {
    /// Project name from `[project].name`.
    pub(crate) name: String,
}

use std::sync::Arc;

use crate::api::bench_config::BenchConfig;
use crate::api::knee_detector::KneeDetector;

/// Builder for autoscale benchmark configuration.
pub struct ApplicationConfigBuilder {
    /// Parsed benchmark configuration.
    pub(crate) config: BenchConfig,
    /// Optional custom knee detector (SPI extension).
    pub(crate) knee_detector: Option<Arc<dyn KneeDetector>>,
}

impl ArchitectureConfigBuilder {
    /// Construct with the given project name.
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Return the project name.
    pub(crate) fn project_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_config_builder_constructs() {
        let _b = ArchitectureConfigBuilder {
            name: "swe-edge-autoscale".to_string(),
        };
    }
}
