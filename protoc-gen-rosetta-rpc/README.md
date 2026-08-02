# protoc-gen-rosetta-rpc

Custom `protoc` compiler plugin for the [godot-rosetta-rpc](../README.md)
cross-language RPC framework. Reads a `CodeGeneratorRequest` on stdin and
writes a `CodeGeneratorResponse` on stdout, generating per-service
interfaces, method descriptors, clients, and adapters. See `tests/golden/`
for exactly what it generates.

## Installing

    cargo install protoc-gen-rosetta-rpc

This installs a `protoc-gen-rosetta-rpc` binary to `~/.cargo/bin` (make sure
it's on `PATH`), which `protoc` can invoke as
`--plugin=protoc-gen-rosetta-rpc=$(command -v protoc-gen-rosetta-rpc)`.

## Usage

To generate bindings for Rust:

```bash
protoc \
  --plugin=protoc-gen-rosetta-rpc=$(which protoc-gen-rosetta-rpc) \
  --rosetta-rpc_out=lang=rust,message_crate=my_messages:out/dir \
  -I proto proto/*.proto
```

To generate bindings for C#:

```bash
protoc \
  --plugin=protoc-gen-rosetta-rpc=$(which protoc-gen-rosetta-rpc) \
  --rosetta-rpc_out=lang=csharp \
  -I proto proto/*.proto
```

To generate bindings for Kotlin:

```bash
protoc \
  --plugin=protoc-gen-rosetta-rpc=$(which protoc-gen-rosetta-rpc) \
  --rosetta-rpc_out=lang=kotlin \
  -I proto proto/*.proto
```

NOTE: This tool does not currently support services spanning multiple proto
packages. Each package requires a separate invocation of `protoc`.

NOTE: Generating bindings in multiple languages typically requires multiple
invocations of `protoc`, as `protoc` plugins can only output to 1 directory per
invocation.

## Supported languages

- Rust
- C#
- Kotlin

See the parent README's "Extending to a new language" section for how to
add support for a new target language (a `LanguageGenerator` implementation
plus an Askama template set under `templates/<language>/`).
