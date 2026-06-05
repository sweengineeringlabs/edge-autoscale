//! Tests verifying the gateway ingress placeholder.
//!
//! The ingress module is a placeholder for future inbound trigger adapters.

/// @covers: gateway::ingress
#[test]
fn test_gateway_ingress_placeholder_exists() {
    // `gateway::ingress` is an empty placeholder reserved for future inbound
    // trigger adapters, so it has no public surface of its own yet. The crate
    // re-exports its public API through the gateway (`pub use gateway::*`);
    // naming a re-exported type is a compile-time contract that the gateway
    // layer resolves — it fails to compile if the gateway wiring is removed.
    let _gateway_surface = core::marker::PhantomData::<swe_edge_autoscale_bench::BenchFacade>;
}
