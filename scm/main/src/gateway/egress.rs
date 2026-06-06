//! Outbound integrations for the bench crate.
//!
//! Adapters for emitting bench reports to external systems (e.g. CI artifact
//! storage, metrics backends, TOML config writers).
//!
//! Re-exports the primary public surface from saf/ for downstream consumers.

pub use crate::saf::*;
