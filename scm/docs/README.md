# swe-edge-autoscale-bench

## WHAT

Empirical load runner and saturation knee detector for swe-edge — drives stepped concurrency load
against any `BenchHandler`, detects the throughput knee, and emits production-ready autoscale config.

Key capabilities:

- **`BenchHandler`** — core trait: `call() → BenchFuture`; abstraction over the handler under test
- **`BenchRunner`** — facade: `run(handler) → Report`; orchestrates concurrency-stepped load probes from warmup through saturation
- **`BenchConfig`** / **`BenchConfigBuilder`** — config from `[bench]` TOML: `concurrency_steps` (e.g., 1→128), `step_duration_secs`, `warmup_secs`, `safety_margin_pct`
- **`KneeDetector`** — trait for saturation detection; implementations: `PlateauDetector`, `InflectionDetector`
- **`ThresholdAdvisor`** — applies safety margin to the detected knee and produces an `AutoscalePolicy`
- **`Report`** — output VO with `summary_table()` and `recommended_policy_toml()` for pasting into application config
- **`AutoscalePolicy`** — recommended `[autoscale]` TOML block (concurrency limit, timeout)

## WHY

| Problem | Solution |
|---------|----------|
| Autoscale thresholds guessed from first principles; production traffic discovers the wrong limits | Empirical load stepping probes actual throughput; knee detection finds saturation before going to production |
| Different knee-detection algorithms needed for different handler shapes (plateau vs. inflection) | `KneeDetector` trait; plug in `PlateauDetector` or `InflectionDetector` without touching the bench runner |
| Safety margin applied manually after a benchmark — inconsistently, in spreadsheets | `ThresholdAdvisor` applies `safety_margin_pct` from config and emits a `AutoscalePolicy` struct; no manual arithmetic |
| Benchmark output is prose; engineer must translate to config | `recommended_policy_toml()` on `Report` returns a ready-to-paste `[autoscale]` block |
| Diamond dep conflicts when bench types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
