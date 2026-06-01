//! Primary processing trait for the bench crate.

/// Primary processing contract: run a benchmark and produce a report.
pub trait Processor: Send + Sync {
    /// Execute the benchmark and return a structured report.
    ///
    /// Returns an error if the bench configuration is invalid (e.g. no
    /// concurrency steps) or if the underlying runner fails fatally.
    fn process(
        &self,
    ) -> futures::future::BoxFuture<
        '_,
        Result<crate::api::outcome::report::Report, crate::api::bench::bench_error::BenchError>,
    >;
}
