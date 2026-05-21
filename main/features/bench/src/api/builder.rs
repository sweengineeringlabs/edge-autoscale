//! Builder for workspace architectural configuration.
///
/// Reads project metadata from `main/config/architecture.toml` at build time.
/// Not typically constructed directly — provided here to satisfy the
/// `<name>.toml → <Name>ConfigBuilder` convention for `architecture.toml`.
#[allow(dead_code)]
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
    fn test_architecture_config_builder_constructs() {
        let _b = ArchitectureConfigBuilder {
            name: "swe-edge-autoscale".to_string(),
        };
    }
}
