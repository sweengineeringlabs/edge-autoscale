# swe-edge-autoscale

Load runner and autoscale threshold calibration for the `swe-edge` stack.

## What

`swe-edge-autoscale-bench` drives concurrency-stepped async load against a real handler,
detects the saturation knee via configurable algorithms (plateau or inflection / Little's Law),
applies a safety margin, and emits a ready-to-paste `[autoscale]` TOML block.

## Workspaces

| Workspace | Crate | Purpose |
|-----------|-------|---------|
| `main/features/bench/` | `swe-edge-autoscale-bench` | Load runner + knee detection library |

## Build

```bash
cd autoscale
cargo build
cargo test
cargo clippy -- -D warnings
```

## Usage

```rust
use std::sync::Arc;
use swe_edge_autoscale_bench::{BenchRunner, BenchConfig, adapt_handler};
use edge_domain::echo_handler;

let config  = BenchConfig::swe_default().unwrap();
let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
let report  = BenchRunner::new(config).run(handler).await.unwrap();

println!("{}", report.summary_table());
// Paste the [autoscale] block from report.recommended_policy_toml() into application.toml
```

## Project Structure

- `main/features/bench/src/api/` — Public traits and value objects (BenchHandler, KneeDetector, LoadRunner, ThresholdAdvisor, LoadReport)
- `main/features/bench/src/core/` — Implementations (TokioLoadRunner, PlateauDetector, InflectionDetector, DefaultThresholdAdvisor)
- `main/features/bench/src/saf/` — Public facade (BenchRunner factory, adapt_handler)
- `main/features/bench/src/spi.rs` — KneeDetector extension hook for custom algorithms

## Related Documents

- [Architecture](docs/3-design/architecture.md)
- [Edge Architecture](../docs/3-architecture/architecture.md)
