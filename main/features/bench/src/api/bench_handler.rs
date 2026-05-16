//! Abstraction over the unit under test for a bench run.

use std::future::Future;
use std::pin::Pin;

use crate::api::bench_error::BenchError;

/// A single async call to the system under test.
///
/// The return type is `Ok(())` on success and `Err(BenchError)` on failure.
/// Errors are counted in [`crate::api::load_report::StepResult::error_count`]
/// but do not abort the step — latency is only recorded for successful calls.
pub type BenchFuture<'a> = Pin<Box<dyn Future<Output = Result<(), BenchError>> + Send + 'a>>;

/// Abstraction over the handler being benchmarked.
///
/// Implement this directly, or use [`crate::saf::adapt_handler`] to wrap an
/// `edge-domain` [`edge_domain::Handler`].
pub trait BenchHandler: Send + Sync {
    /// Invoke the handler once. Called repeatedly across all concurrency tasks.
    fn call(&self) -> BenchFuture<'_>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysOk;

    impl BenchHandler for AlwaysOk {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Ok(()) })
        }
    }

    #[tokio::test]
    async fn test_call_returns_ok_for_successful_handler() {
        let h = AlwaysOk;
        assert!(h.call().await.is_ok());
    }

    struct AlwaysFail;

    impl BenchHandler for AlwaysFail {
        fn call(&self) -> BenchFuture<'_> {
            Box::pin(async { Err(BenchError::StepFailed("forced".into())) })
        }
    }

    #[tokio::test]
    async fn test_call_returns_err_for_failing_handler() {
        let h = AlwaysFail;
        assert!(h.call().await.is_err());
    }
}
