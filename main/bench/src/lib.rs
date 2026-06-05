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
//! use swe_edge_autoscale_bench::{BenchFacade, BenchConfig};
//! use edge_domain::Domain;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config  = BenchConfig::default();
//!     let handler = BenchFacade::adapt_handler(
//!         Domain::echo_handler("ping", "/ping"),
//!         || "ping".to_string(),
//!     );
//!     let report  = BenchFacade::create_bench_runner(config).run(handler).await.unwrap();
//!     println!("{}", report.summary_table());
//! }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::egress::*;
