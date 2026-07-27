# godot-rosetta-rpc

The handwritten Rust runtime for the [godot-rosetta-rpc](../README.md) cross-language
RPC framework: `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient` (Godot-free, unit
testable with plain `cargo test`), plus a `godot_support` module of thin gdext-facing
helper functions. Never depends on any project-specific generated code — see the
parent README's "Why no dependency cycle" section.

## Dependency versions

The `prost::Message` and `godot` types appear directly in this crate's public API.
`prost` and `godot` versions live in `[workspace.dependencies]` in the workspace root
`Cargo.toml`, so every crate in this workspace stays in lockstep automatically.
However, since Cargo treats every pre-1.0 (`0.x`) release as its own incompatible major version,
a consuming project pinned to a different version of either crate will be incompatible with this crate.

We will periodically bump the pinned version here and republish the crate, rather than widen the version requirement speculatively.
