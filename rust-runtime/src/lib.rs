//! Handwritten Rust runtime for the godot-rosetta-rpc cross-language RPC
//! framework. `descriptor`, `registry`, `client`, and `error` are pure Rust —
//! never touch a gdext type, and are unit-testable with plain `cargo test`
//! regardless of whether a Godot engine is running. `godot_support` is the
//! only module that talks to gdext, and it never references any
//! project-specific generated type — see the crate-level README for why that
//! matters.

mod client;
mod descriptor;
mod error;
pub mod godot_support;
mod registry;

pub use client::RpcClient;
pub use descriptor::RpcMethodDescriptor;
pub use error::RpcError;
pub use registry::{ErasedAdapter, ServiceRegistry};
