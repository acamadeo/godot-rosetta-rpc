# godot-rosetta-rpc

The Rust runtime for the [godot-rosetta-rpc](../README.md) cross-language
RPC framework. This crates allows you to implement protobuf RPC services in Rust,
making the services callable from any supported programming language in your Godot
codebase. It also allows you to call services implemented
in other languages from Rust.

This crate works in tandem with the protobuf compiler plugin
[protoc-gen-rosetta-rpc](https://crates.io/crates/protoc-gen-rosetta-rpc),
which generates language bindings for your custom RPC services. This crate primarily
provides the Rust implementation of the generated types, e.g.
`RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient`.

It comes with a `godot_support` module, which integrates with Godot to provide
handles to call RPC services from Rust.

## Installing

```toml
[dependencies]
godot-rosetta-rpc = "0.1"
```
