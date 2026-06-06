//! Recommended autoscale thresholds derived from the knee step.

/// Recommended autoscale thresholds derived from the knee step.
///
/// Emitted by [`Report`] after applying the `safety_margin_pct` to the
/// measured knee values. Paste the output of [`Report::recommended_policy_toml`]
/// into `application.toml` under `[autoscale]`.
///
/// [`Report`]: crate::Report
/// [`Report::recommended_policy_toml`]: crate::Report::recommended_policy_toml
///
/// # Examples
///
/// ```rust
/// use swe_edge_autoscale_bench::AutoscalePolicy;
///
/// let policy = AutoscalePolicy {
///     requests_active_max: 22,
///     requests_per_sec_max: 294,
///     latency_p99_ms_max: 84.0,
/// };
///
/// assert_eq!(policy.requests_active_max, 22);
/// assert!(policy.latency_p99_ms_max > 0.0);
/// ```
#[derive(Debug, Clone)]
pub struct AutoscalePolicy {
    /// `requests_active_max` for the `[autoscale]` TOML block.
    pub requests_active_max: usize,
    /// `requests_per_sec_max` for the `[autoscale]` TOML block.
    pub requests_per_sec_max: u64,
    /// `latency_p99_ms_max` for the `[autoscale]` TOML block.
    pub latency_p99_ms_max: f64,
}
