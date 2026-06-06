#!/usr/bin/env bash
set -euo pipefail
cargo install cargo-audit --locked 2>/dev/null || true
cargo build -p swe-edge-autoscale-bench
