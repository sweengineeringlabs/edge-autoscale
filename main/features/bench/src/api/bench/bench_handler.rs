//! Abstraction over the unit under test for a bench run.

use crate::api::bench::bench_future::BenchFuture;

/// Abstraction over the handler being benchmarked.
///
/// Implement this directly, or use [`crate::saf::BenchFacade::adapt_handler`] to wrap an
/// `edge-domain` [`edge_domain::Handler`].
pub trait BenchHandler: Send + Sync {
    /// Invoke the handler once. Called repeatedly across all concurrency tasks.
    fn call(&self) -> BenchFuture<'_>;
}
