//! Application-level config builder for the bench crate.
//!
//! Bridges `swe-edge-configbuilder` with [`BenchConfig`] so callers can
//! load configuration through the standard config loader pipeline.

use crate::api::vo::bench_config::BenchConfig;

/// Loads [`BenchConfig`] from the standard config loader pipeline.
///
/// Wraps the `swe_edge_configbuilder` pipeline so that consumers get a
/// single entry point with no knowledge of the underlying TOML mechanics.
pub struct ApplicationConfigBuilder {
    loader: swe_edge_configbuilder::SectionLoaderImpl,
}

impl ApplicationConfigBuilder {
    /// Create a builder from an already-constructed `SectionLoaderImpl`.
    pub fn new(loader: swe_edge_configbuilder::SectionLoaderImpl) -> Self {
        Self { loader }
    }

    /// Load and return the [`BenchConfig`] from the config pipeline.
    pub fn build(self) -> Result<BenchConfig, swe_edge_configbuilder::ConfigError> {
        use swe_edge_configbuilder::ConfigSection as _;
        BenchConfig::load(&self.loader)
    }
}
