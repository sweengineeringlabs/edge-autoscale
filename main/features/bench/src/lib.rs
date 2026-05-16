//! # swe-edge-autoscale-bench
//!
//! Load runner and saturation knee detector for autoscale threshold calibration.
//!
//! Drives concurrency-stepped async load against a real [`BenchHandler`],
//! detects the saturation knee, applies a safety margin, and emits a
//! ready-to-paste `[autoscale]` TOML block.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use swe_edge_autoscale_bench::{BenchRunner, BenchConfig, adapt_handler};
//! use edge_domain::echo_handler;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config  = BenchConfig::swe_default().unwrap();
//!     let handler = adapt_handler(echo_handler("ping", "/ping"), || "ping".to_string());
//!     let report  = BenchRunner::new(config).run(handler).await.unwrap();
//!     println!("{}", report.summary_table());
//! }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use saf::*;
