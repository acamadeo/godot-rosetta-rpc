# godot-rosetta-rpc

A lightweight, in-process, cross-language RPC framework for Godot projects that
mix implementation languages (initially Rust and Kotlin). `.proto` files are the
single source of truth: a custom `protoc` plugin generates thin per-language
glue (service interfaces, method descriptors, clients, adapters), small
handwritten runtime libraries own the real behavior, and one tiny
`RpcRuntime.gd` autoload routes calls between language runtimes inside a single
Godot process. This is **not** networked gRPC — everything stays in-process, and
only byte arrays cross the Rust/Kotlin/GDScript boundary.

See [`../godot-rosetta-rpc-design.md`](../godot-rosetta-rpc-design.md) for the
full design spec this implements.

## Layout

- `protoc-gen-rosetta-rpc/` — the custom protoc plugin (Rust binary,
  `cargo install`-able). Reads a `CodeGeneratorRequest` on stdin, generates
  per-service interfaces, method descriptors, clients, and adapters (plus a
  per-invocation `ServiceFactory` and `Bootstrap`), and writes a
  `CodeGeneratorResponse` on stdout. See its own `tests/golden/` for exactly
  what it generates.
- `rust-runtime/` — Cargo package `godot-rosetta-rpc` (imported as
  `godot_rosetta_rpc`). The handwritten Rust runtime: `RpcMethodDescriptor`,
  `ServiceRegistry`, `RpcClient` (Godot-free, unit-testable with plain
  `cargo test`), plus a small `godot_support` module of gdext-facing helper
  functions. Never depends on generated code — see "Why no dependency cycle"
  below.
- `kotlin-runtime/` — Gradle module, artifact id `godot-rosetta-rpc`. Mirrors
  the Rust runtime for the JVM/Kotlin side.
- `godot/RpcRuntime.gd` — the one hand-authored, hard-coded autoload script.
  Never generated. Bootstraps each language runtime and routes calls between
  them by service id.
- `install/install.py` / `install/uninstall.py` — pure standard-library Python 3
  scripts that copy `RpcRuntime.gd` into a target Godot project and
  register/deregister it as an autoload in that project's `project.godot`.
- `scripts/run_godot_tests.sh` — runs the Godot-dependent parts of the test
  suite if a `godot` or `godot4` binary is found on `PATH`; otherwise skips
  cleanly.
- `example/` — a minimal fixture project (package `rosetta.example`,
  deliberately not reusing the parent repo's `abacus` domain) proving the whole
  pipeline end-to-end: a `Clock` service implemented in Rust, called by a
  `GameService` implemented in Kotlin, purely through `RpcRuntime.gd`.

## Why no dependency cycle

Generated adapters reference concrete service implementations, which only a
specific project knows about. If the runtime library depended on generated code
(or vice versa in a way that closed the loop), you'd get a cycle. Instead:

- `rust-runtime` / `kotlin-runtime` depend on nothing project-specific. They
  provide `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient`, and thin
  Godot-facing helper functions only.
- The concrete `RustRuntime` / `KotlinRuntime` classes that Godot actually
  instantiates (via `ClassDB.instantiate(...)`) are **generated per project** by
  the plugin, in the project's own `Bootstrap` output — because only they are
  allowed to reference that project's generated `Bootstrap::register(...)` and
  concrete service implementations.

This also means both runtime libraries can eventually be published standalone
(crates.io / Maven) without restructuring.

## Extending to a new language

1. A standard protobuf code generator for that language (message types only).
2. A small runtime library implementing `RpcMethodDescriptor`,
   `ServiceRegistry`, `RpcClient`, mirroring `rust-runtime`/`kotlin-runtime`.
3. A new `LanguageGenerator` implementation + Askama template set in
   `protoc-gen-rosetta-rpc/templates/<language>/`.

No changes are required to `.proto` definitions, `RpcRuntime.gd`, or any other
language's generator or runtime.

## Known issues / possible feature requests

- **No naming-collision detection.** The plugin has everything needed
  (`TypeResolver` in `ir.rs`) to detect a service name colliding with an
  existing message/enum name in the same proto package, but doesn't check it --
  the failure mode is a confusing downstream Rust/Kotlin compile error rather
  than a clear `protoc`-level error. Worth a validation pass in
  `generator.rs`/`ir.rs` that returns a `CodeGeneratorResponse.error` on
  collision, plus documenting _why_ this is possible (Kotlin glue reuses the
  literal proto package as its own package).

- **Limited to single-package-per invocation**: the tool fails when used on
  services that span multiple proto packages. The fix would involve keying
  aggregate output per-package.

## Publishing status

| Root directory           | Status                                                                              |
| ------------------------ | ----------------------------------------------------------------------------------- |
| `protoc-gen-rosetta-rpc` | ✅ Published - https://crates.io/crates/protoc-gen-rosetta-rpc                      |
| `rust-runtime`           | ✅ Published - https://crates.io/crates/godot-rosetta-rpc                           |
| `kotlin-runtime`         | ✅ Published - https://repo1.maven.org/maven2/io/github/acamadeo/godot-rosetta-rpc/ |
