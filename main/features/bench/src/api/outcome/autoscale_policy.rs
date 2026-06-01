//! Recommended autoscale thresholds derived from the knee step.

/// Recommended autoscale thresholds derived from the knee step.
#[derive(Debug, Clone)]
pub struct AutoscalePolicy {
    /// `requests_active_max` for the `[autoscale]` TOML block.
    pub requests_active_max: usize,
    /// `requests_per_sec_max` for the `[autoscale]` TOML block.
    pub requests_per_sec_max: u64,
    /// `latency_p99_ms_max` for the `[autoscale]` TOML block.
    pub latency_p99_ms_max: f64,
}
