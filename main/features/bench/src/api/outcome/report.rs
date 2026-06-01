//! Full output of a bench run.

use super::{autoscale_policy::AutoscalePolicy, step_result::StepResult};

/// Full output of a bench run.
#[derive(Debug)]
pub struct Report {
    /// One entry per concurrency step, in probe order.
    pub steps: Vec<StepResult>,
    /// Index into `steps` of the detected knee. `None` if no knee was found.
    pub knee_index: Option<usize>,
    /// Recommended autoscale thresholds after applying the safety margin.
    pub policy: AutoscalePolicy,
}

impl Report {
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
