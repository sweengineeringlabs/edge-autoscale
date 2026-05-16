//! Error type for swe-edge-autoscale-bench.

/// Errors produced by the bench crate.
#[derive(Debug, thiserror::Error)]
pub enum BenchError {
    /// TOML config failed to deserialize as [`super::bench_config::BenchConfig`].
    #[error("swe_edge_autoscale_bench: config parse failed — {0}")]
    ConfigParseFailed(String),

    /// A single handler call returned an error during a measurement step.
    #[error("swe_edge_autoscale_bench: handler call failed — {0}")]
    StepFailed(String),

    /// `concurrency_steps` is empty; at least one step is required.
    #[error("swe_edge_autoscale_bench: concurrency_steps must contain at least one entry")]
    NoSteps,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parse_failed_display_names_crate() {
        let e = BenchError::ConfigParseFailed("bad toml".into());
        assert!(e.to_string().contains("swe_edge_autoscale_bench"));
        assert!(e.to_string().contains("bad toml"));
    }

    #[test]
    fn test_step_failed_display_includes_message() {
        let e = BenchError::StepFailed("timeout".into());
        assert!(e.to_string().contains("timeout"));
    }

    #[test]
    fn test_no_steps_display_mentions_concurrency_steps() {
        let e = BenchError::NoSteps;
        assert!(e.to_string().contains("concurrency_steps"));
    }
}
