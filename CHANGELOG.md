# Changelog

All notable changes to `swe-edge-autoscale` will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-05-16

### Added

- `swe-edge-autoscale-bench` library crate — SEA-compliant workspace scaffolding
- `BenchConfig` — TOML-deserializable config (concurrency steps, duration, warmup, safety margin)
- `TokioLoadRunner` — N concurrent Tokio tasks per step, sorted-Vec latency histogram
- `PlateauDetector` — knee = first step where RPS growth falls below threshold
- `InflectionDetector` — knee = first step where Δp99ms/ΔRPS ratio exceeds threshold (Little's Law)
- `DefaultThresholdAdvisor` — applies safety margin to all three autoscale dimensions
- `BenchRunner` factory in `saf/` — selects algorithm from config, runs all steps, emits `LoadReport`
- `adapt_handler` — wraps `edge-domain` `Handler<Req, Resp>` as `BenchHandler`
- `KneeDetector` SPI extension hook for custom detection algorithms
- `LoadReport::summary_table()`, `recommended_policy_toml()`, `to_json()` output modes
- 37 tests (30 unit, 7 integration against real `echo_handler`)
