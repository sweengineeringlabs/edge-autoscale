//! Public facade type for the autoscale-bench crate.

/// Public facade for the autoscale-bench crate.
///
/// All factory operations are methods on `BenchFacade` to satisfy the
/// SEA requirement that saf/ has no free-standing functions.
///
/// The method bodies live in `saf/` since they must wire concrete
/// `core/` types (which api/ is not permitted to import).
pub struct BenchFacade;
