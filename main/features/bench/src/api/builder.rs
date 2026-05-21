//! Builders for workspace configuration (type declarations per SEA rule 160).

use crate::api::bench_config::BenchConfig;
use crate::api::knee_detector::KneeDetector;
use std::sync::Arc;

/// Builder for workspace architectural configuration.
///
/// Reads project metadata from `main/config/architecture.toml` at build time.
/// Not typically constructed directly — provided here to satisfy the
/// `<name>.toml → <Name>ConfigBuilder` convention for `architecture.toml`.
pub struct ArchitectureConfigBuilder {
    /// Project name from `[project].name`.
    pub(crate) name: String,
}

/// Builder for benchmark application configuration.
///
/// Construct via [`crate::saf::builder`] (loads config from TOML).
/// Finalize with [`ApplicationConfigBuilder::build`].
pub struct ApplicationConfigBuilder {
    /// The parsed application configuration.
    pub(crate) config: BenchConfig,
    /// Optional custom knee detector implementation (SPI extension).
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

    #[test]
    fn test_application_config_builder_has_no_knee_detector_by_default() {
        use crate::api::bench_config::BenchConfig;
        let config = BenchConfig::from_config("").unwrap();
        let b = ApplicationConfigBuilder {
            config,
            knee_detector: None,
        };
        assert!(b.knee_detector.is_none());
    }
}
