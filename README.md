# godot-rosetta-rpc

A lightweight, in-process, cross-language framework for Godot projects that
mix implementation languages. This project leverages
[Protocol buffer services](https://protobuf.dev/programming-guides/proto3/#services)
to define interfaces that can then be implemented by and called from supported
languages within your Godot codebase.

Note that although the title references 'RPC' (Remote procedure call), this framework
is not meant for client-server networking. It merely re-uses an interface language
used by such networking tools (like
[gRPC](https://grpc.io/docs/what-is-grpc/core-concepts/#overview)) to define interfaces
across language boundaries. The 'client' and 'server', in this case, are classes in
your Godot codebase that may be written in different languages.

Some benefits this tool can provide:

- **Promotes service-oriented architecture**: You can easily define many narrowly-scoped
  singleton services accessible to your entire Godot project from any language (e.g.
  `SaveManager`, `Weather`, `HttpClient`, etc).
- **Facilitates optimization**: Do you have any code that is too
  performance-critical to be written in GDScript or C#? You can implement this logic
  in more performant languages like Rust, C++, or Go, and with `godot-rosetta-rpc`,
  you can have it easily interop with the rest of your game.

## Example

Define your cross-language service boundaries using `.proto` files:

```proto
message NextUint64Request {
  enum RandomSystem {
    AUDIO = 1;
    GRAPHICS = 2;
    GAMEPLAY = 3;
  }

  RandomSystem system = 1;
}

message NextUint64Response {
  uint64 value = 1;
}

service RngService {
  rpc NextUint64(NextUint64Request) returns (NextUint64Response);
}
```

Then implement it in your desired language:

```rust
// Rust implementation
pub struct RngServiceImpl {
  // ...
}

impl RngService for RngServiceImpl {
    fn next_uint64(&self, request: NextUint64Request) -> NextUint64Response {
        NextUint64Response { value: self.rngs.get(request.system).nextUint64() }
    }
}
```

And call it from any other service or `Node`, written in any language:

```kotlin
// Kotlin node
@RegisterClass
class MyNode : Node2D() {

  @RegisterFunction
  override fun _ready() {
    val services = GeneratedServiceFactory(GodotSupport.makeRpcClient(this))
    val rngService = services.rngService()
    val response = rngService.nextUint64(nextUint64Request {
      system = RandomSystem.GAMEPLAY
    })
    GD.print("Random number: ${response.value}")
  }
}
```

See the `example/` directory for a more fleshed-out example.

## Layout

- `protoc-gen-rosetta-rpc/` - this is a custom `protoc` compiler plugin
  that generates the language bindings for your defined services. This is
  a Rust binary, installable through `cargo`.

- Language runtimes - these are small language-specific libraries that
  do stuff. These libraries offer a similar interface, including `RpcClient`,
  `RpcMethodDescriptor`, `ServiceRegistry`, and a factory method that interfaces
  with Godot.
  - `rust-runtime/` - Rust language runtime, available as the Cargo package
    `godot-rosetta-rpc`.
  - `kotlin-runtime/` - Kotlin language runtime, available as Gradle module
    `io.github.acamadeo:godot-rosetta-rpc`.

- `godot/RpcRuntime.gd` - an autoload script written in GDScript that intercepts
  service requests and delegates them to the proper language runtime. This is
  the only autoload `godot-rosetta-rpc` requires. It can be installed into your
  Godot project through `install/install.py`.

- `example/` — a minimal fixture project, showcasing the whole pipeline
  end-to-end: a `Clock` service implemented in Rust, called by a
  `GameService` implemented in Kotlin and a `Profiler` service implemented in
  Rust.

## Installing

1.  [Install](https://protobuf.dev/installation/) `protoc`, the protocol buffer
    compiler.

1.  Install `protoc-gen-rosetta-rpc`, the custom `protoc` plugin for this framework:

    ```bash
    cargo install protoc-gen-rosetta-rpc
    ```

1.  Run the install script to set up your Godot codebase with the `RpcRuntime.gd`
    autoload:

    ```bash
    python3 install/install.py /path/to/godot/project
    ```

## Usage

1.  Invoke `protoc` on the `.proto` files defining your services with the
    `protoc-gen-rosetta-rpc` plugin enabled. This generates bindings for your
    services in the desired language:

    ```bash
    protoc \
     --plugin=protoc-gen-rosetta-rpc=$(which protoc-gen-rosetta-rpc) \
     --rosetta-rpc_out=lang=rust,message_crate=my_messages:out/dir \
     -I proto proto/\*.proto
    ```

1.  Implement each RPC service. Each `service Foo {}` from your proto will be exposed
    as a `interface Foo {}`, which can be implemented anywhere in your codebase. Within
    a single service, every method must be implemented in the same language.

1.  Link the service implementation through a `rpcimpls` module. This involves
    implementing the `ServiceImplementations` interface, which tells the language
    runtime and, ultimately, `RpcRuntime.gd` which services are implemented in which
    languages.

1.  Call the RPC service. If you wish to call an RPC service from a Godot `Node`, you can
    do so by constructing a `GeneratedServiceFactory` with an `RpcClient`. It provides methods
    to access and call each of your defined RPC services.

    If you wish to call from an RPC service implementation, you can pass
    `GeneratedServiceFactory` to your implementation and use it in the same way to invoke
    other services.

See each language runtime's README with more specific instructions on how to use
`godot-rosetta-rpc` within that language:

| Language | Docs                               |
| -------- | ---------------------------------- |
| Kotlin   | [README](kotlin-runtime/README.md) |
| Rust     | [README](rust-runtime/README.md)   |

## Extending to a new language

This involves:

1.  Having a standard protobuf code generator for that language (message types only).
2.  Adding a small runtime library implementing `RpcMethodDescriptor`,
    `ServiceRegistry`, `RpcClient`, mirroring `rust-runtime`/`kotlin-runtime`.
3.  Adding a new `LanguageGenerator` implementation + Askama template set in
    `protoc-gen-rosetta-rpc/templates/<language>/`.

No changes are required to `.proto` definitions, `RpcRuntime.gd`, or any other
language's generator or runtime.

## Limitations / known issues

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
