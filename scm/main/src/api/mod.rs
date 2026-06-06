//! API layer — public trait and type definitions.
//!
//! Theme dirs (`advisor/`, `knee/`, `processor/`, `step/`, `validator/`)
//! carry the inner SEA layout; crate-level `traits/`, `types/`, `error/`,
//! and `vo/` hold cross-theme contracts consumed by two or more themes.

pub(crate) mod advisor;
pub(crate) mod error;
pub(crate) mod knee;
pub(crate) mod processor;
pub(crate) mod step;
pub(crate) mod traits;
pub(crate) mod types;
pub(crate) mod validator;
pub(crate) mod vo;
