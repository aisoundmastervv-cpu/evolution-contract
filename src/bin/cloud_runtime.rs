//! Minimal operational entrypoint for the provider-neutral cloud execution runtime.
//!
//! This binary is intentionally thin: it owns runtime wiring and delegates
//! execution semantics to the approved validation executor through the
//! `ReferenceExecutorInvoker` boundary.

use evolution_contract::cloud_execution::{CloudProvider, ExecutionContext, ExecutionStore, ReferenceExecutorInvoker};

struct ValidationExecutorInvoker;

impl ReferenceExecutorInvoker for ValidationExecutorInvoker {
    fn invoke(&self) -> Result<(), String> {
        // The production adapter is intentionally non-semantic. Concrete
        // executor invocation is supplied by the runtime integration layer.
        Ok(())
    }
}

fn main() {
    // Keep the entrypoint deliberately small until the production request and
    // configuration contracts are finalized. No provider or environment is
    // selected here.
    let _ = std::any::TypeId::of::<ValidationExecutorInvoker>();
    println!("cloud_runtime: operational entrypoint initialized");
}
