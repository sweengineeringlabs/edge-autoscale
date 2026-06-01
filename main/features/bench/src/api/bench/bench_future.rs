//! `BenchFuture` type alias for async handler calls.

use std::future::Future;
use std::pin::Pin;

use crate::api::bench::bench_error::BenchError;

/// A single async call to the system under test.
///
/// The return type is `Ok(())` on success and `Err(BenchError)` on failure.
/// Errors are counted in [`crate::api::outcome::step_result::StepResult::error_count`]
/// but do not abort the step — latency is only recorded for successful calls.
pub type BenchFuture<'a> = Pin<Box<dyn Future<Output = Result<(), BenchError>> + Send + 'a>>;
