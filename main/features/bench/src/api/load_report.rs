//! Result types produced by a bench run.

/// Measurements for a single concurrency step.
#[derive(Debug, Clone)]
pub struct StepResult {
    /// Number of concurrent tasks during this step.
    pub concurrency: usize,
    /// Requests per second over the measurement window.
    pub rps: f64,
    /// Median latency in milliseconds.
    pub p50_ms: f64,
    /// 95th-percentile latency in milliseconds.
    pub p95_ms: f64,
    /// 99th-percentile latency in milliseconds.
    pub p99_ms: f64,
    /// 99.9th-percentile latency in milliseconds.
    pub p99_9_ms: f64,
    /// Number of calls that returned an error during the measurement window.
    pub error_count: u64,
}

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

/// Full output of a bench run.
#[derive(Debug)]
pub struct LoadReport {
    /// One entry per concurrency step, in probe order.
    pub steps: Vec<StepResult>,
    /// Index into `steps` of the detected knee. `None` if no knee was found.
    pub knee_index: Option<usize>,
    /// Recommended autoscale thresholds after applying the safety margin.
    pub policy: AutoscalePolicy,
}

impl LoadReport {
    /// Render a human-readable table followed by a ready-to-paste TOML block.
    pub fn summary_table(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{:<12} | {:<12} | {:<7} | {:<7} | {:<8} | {:<10} | {}\n",
            "Concurrency", "RPS", "p50ms", "p95ms", "p99ms", "p99.9ms", "Errors"
        ));
        out.push_str(&"-".repeat(73));
        out.push('\n');
        for (i, s) in self.steps.iter().enumerate() {
            let marker = if self.knee_index == Some(i) {
                "  <- knee"
            } else {
                ""
            };
            out.push_str(&format!(
                "{:<12} | {:<12.0} | {:<7.2} | {:<7.2} | {:<8.2} | {:<10.2} | {}{}\n",
                s.concurrency,
                s.rps,
                s.p50_ms,
                s.p95_ms,
                s.p99_ms,
                s.p99_9_ms,
                s.error_count,
                marker
            ));
        }
        out.push('\n');
        match self.knee_index {
            Some(idx) => out.push_str(&format!(
                "Knee: concurrency={}\n\n",
                self.steps[idx].concurrency
            )),
            None => out.push_str("Knee: not detected — increase the concurrency_steps range\n\n"),
        }
        out.push_str(&self.recommended_policy_toml());
        out
    }

    /// Emit a ready-to-paste `[autoscale]` TOML block.
    pub fn recommended_policy_toml(&self) -> String {
        let p = &self.policy;
        format!(
            "# Paste into application.toml:\n[autoscale]\nrequests_active_max  = {}\nrequests_per_sec_max = {}\nlatency_p99_ms_max   = {:.2}\n",
            p.requests_active_max, p.requests_per_sec_max, p.latency_p99_ms_max
        )
    }

    /// Emit the report as a JSON string for CI artifact storage.
    pub fn to_json(&self) -> String {
        let knee_concurrency = self
            .knee_index
            .map(|i| self.steps[i].concurrency.to_string())
            .unwrap_or_else(|| "null".into());
        let steps_json: Vec<String> = self
            .steps
            .iter()
            .map(|s| {
                format!(
                    r#"{{"concurrency":{},"rps":{:.1},"p50_ms":{:.3},"p95_ms":{:.3},"p99_ms":{:.3},"p99_9_ms":{:.3},"error_count":{}}}"#,
                    s.concurrency, s.rps, s.p50_ms, s.p95_ms, s.p99_ms, s.p99_9_ms, s.error_count
                )
            })
            .collect();
        format!(
            r#"{{"knee_concurrency":{},"policy":{{"requests_active_max":{},"requests_per_sec_max":{},"latency_p99_ms_max":{:.2}}},"steps":[{}]}}"#,
            knee_concurrency,
            self.policy.requests_active_max,
            self.policy.requests_per_sec_max,
            self.policy.latency_p99_ms_max,
            steps_json.join(",")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(knee_index: Option<usize>) -> LoadReport {
        LoadReport {
            steps: vec![
                StepResult {
                    concurrency: 1,
                    rps: 1000.0,
                    p50_ms: 0.5,
                    p95_ms: 0.8,
                    p99_ms: 1.0,
                    p99_9_ms: 1.5,
                    error_count: 0,
                },
                StepResult {
                    concurrency: 2,
                    rps: 1800.0,
                    p50_ms: 0.6,
                    p95_ms: 0.9,
                    p99_ms: 4.0,
                    p99_9_ms: 8.0,
                    error_count: 1,
                },
            ],
            knee_index,
            policy: AutoscalePolicy {
                requests_active_max: 1,
                requests_per_sec_max: 700,
                latency_p99_ms_max: 0.70,
            },
        }
    }

    #[test]
    fn test_summary_table_contains_knee_marker_when_knee_detected() {
        let r = make_report(Some(1));
        let t = r.summary_table();
        assert!(t.contains("<- knee"));
    }

    #[test]
    fn test_summary_table_contains_not_detected_when_knee_is_none() {
        let r = make_report(None);
        let t = r.summary_table();
        assert!(t.contains("not detected"));
    }

    #[test]
    fn test_recommended_policy_toml_contains_autoscale_section() {
        let r = make_report(None);
        let toml = r.recommended_policy_toml();
        assert!(toml.contains("[autoscale]"));
        assert!(toml.contains("requests_active_max"));
        assert!(toml.contains("latency_p99_ms_max"));
    }

    #[test]
    fn test_to_json_contains_policy_and_steps_keys() {
        let r = make_report(Some(0));
        let j = r.to_json();
        assert!(j.contains("\"policy\""));
        assert!(j.contains("\"steps\""));
        assert!(j.contains("\"knee_concurrency\""));
    }

    #[test]
    fn test_to_json_encodes_null_knee_when_not_detected() {
        let r = make_report(None);
        assert!(r.to_json().contains("\"knee_concurrency\":null"));
    }
}
