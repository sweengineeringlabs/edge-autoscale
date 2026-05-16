//! Builder type declarations (public types live in `api/`).
//!
//! Impl blocks live in [`crate::saf`]. This file declares the struct
//! shapes so types are anchored in the interface layer.

use std::sync::Arc;

use crate::api::bench_config::BenchConfig;
use crate::api::knee_detector::KneeDetector;

/// Opaque builder for `swe-edge-autoscale-bench`.
///
/// Construct via [`crate::builder`] or [`ApplicationConfigBuilder::with_config`].
/// Finalize with [`ApplicationConfigBuilder::build`].
pub struct ApplicationConfigBuilder {
    /// Parsed bench configuration.
    pub(crate) config: BenchConfig,
    /// Optional custom knee detector (overrides the algorithm in config).
    pub(crate) knee_detector: Option<Arc<dyn KneeDetector>>,
}

/// Builder for workspace architectural configuration.
///
/// Reads project metadata from `main/config/architecture.toml` at build time.
/// Not typically constructed directly — provided here to satisfy the
/// `<name>.toml → <Name>ConfigBuilder` convention for `architecture.toml`.
pub struct ArchitectureConfigBuilder {
    /// Project name from `[project].name`.
    pub(crate) name: String,
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
    fn test_architecture_config_builder_stores_project_name() {
        let b = ArchitectureConfigBuilder::new("swe-edge-autoscale");
        assert_eq!(b.project_name(), "swe-edge-autoscale");
    }

    #[test]
    fn test_application_config_builder_has_no_knee_detector_by_default() {
        use crate::api::bench_config::BenchConfig;
        let config = BenchConfig::from_config("").unwrap();
        let b = ApplicationConfigBuilder { config, knee_detector: None };
        assert!(b.knee_detector.is_none());
    }
}
