//! Builder for workspace architectural configuration.
//!
//! Impl blocks live in the `saf` layer. Struct shapes are declared here so
//! types are anchored in the interface layer per SEA rule 160.

/// Builder for workspace architectural configuration.
///
/// Reads project metadata from `main/config/architecture.toml` at build time.
/// Not typically constructed directly — provided here to satisfy the
/// `<name>.toml → <Name>ConfigBuilder` convention for `architecture.toml`.
#[derive(Debug, Default)]
pub struct ArchitectureConfigBuilder {
    _private: (),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architecture_config_builder_constructs() {
        let _b = ArchitectureConfigBuilder::default();
    }
}
