# godot-rosetta-rpc

[![crates.io - protoc-gen-rosetta-rpc](https://img.shields.io/crates/v/protoc-gen-rosetta-rpc.svg?label=protoc-gen-rosetta-rpc)](https://crates.io/crates/protoc-gen-rosetta-rpc)
[![crates.io - godot-rosetta-rpc](https://img.shields.io/crates/v/godot-rosetta-rpc.svg?label=rust-runtime)](https://crates.io/crates/godot-rosetta-rpc)
[![Maven Central - godot-rosetta-rpc](https://img.shields.io/maven-central/v/io.github.acamadeo/godot-rosetta-rpc.svg?label=kotlin-runtime)](https://repo1.maven.org/maven2/io/github/acamadeo/godot-rosetta-rpc/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A lightweight, in-process, cross-language framework for Godot projects that mix
implementation languages. It uses
[Protocol buffer services](https://protobuf.dev/programming-guides/proto3/#services)
to define interfaces that can be implemented in one language and called from any
other, within your Godot codebase.

The title references 'RPC' (remote procedure call) because it reuses the interface
language of networking tools like
[gRPC](https://grpc.io/docs/what-is-grpc/core-concepts/#overview) — however, this tool is not specifically for
client-server networking. The 'client' and 'server' here are just classes in your
Godot codebase that happen to be written in different languages.

Some benefits this tool can provide:

- **Promotes service-oriented architecture**: You can easily define many narrowly-scoped
  singleton services accessible to your entire Godot project from any language (e.g.
  `SaveManager`, `WeatherSystem`, `HttpClient`, etc).
- **Facilitates optimization**: Code that's too performance-critical for GDScript or C#
  can be implemented in a faster language like Rust, C++, or Go, and still interop
  easily with the rest of your game through `godot-rosetta-rpc`.

## Table of contents

- [Example](#example)
- [Layout](#layout)
- [Installing](#installing)
- [Usage](#usage)
- [Extending to a new language](#extending-to-a-new-language)
- [Limitations / known issues](#limitations--known-issues)

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

See the [example/](example/) directory for a more fleshed-out example.

## Layout

- [`protoc-gen-rosetta-rpc/`](protoc-gen-rosetta-rpc/) - this is a custom `protoc` compiler plugin
  that generates the language bindings for your defined services. This is
  a Rust binary, installable through `cargo`.

- Language runtimes - these are small language-specific libraries that
  bridge the gap between your generated RPC service code and Godot. These libraries
  offer a similar interface, including `RpcClient`, `RpcMethodDescriptor`,
  `ServiceRegistry`, and a factory method that interfaces with Godot.
  - [`rust-runtime/`](rust-runtime/) - Rust language runtime, available as the Cargo package
    `godot-rosetta-rpc`.
  - [`csharp-runtime/`](csharp-runtime/) - C# language runtime, packaged as the NuGet package
    `GodotRosettaRpc`.
  - [`kotlin-runtime/`](kotlin-runtime/) - Kotlin language runtime, available as Gradle module
    `io.github.acamadeo:godot-rosetta-rpc`.

- [`godot/RpcRuntime.gd`](godot/RpcRuntime.gd) - an autoload script written in GDScript that intercepts
  service requests and delegates them to the proper language runtime. This is
  the only autoload `godot-rosetta-rpc` requires. It can be installed into your
  Godot project through `install/install.py`.

- [`example/`](example/) - a minimal fixture project, showcasing the whole pipeline
  end-to-end: a `Clock` service implemented in Rust, called by a
  `Profiler` service (also Rust), a `GameService` implemented in Kotlin, and
  an `Achievements` service implemented in C#.

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

    If your project generates the Kotlin/C# runtime into non-default locations, override
    the directories `RpcRuntime.gd` scans for them with `--csharp-rpc-root`/
    `--kotlin-gdj-root` (both default to the conventions this repo's own
    `example/` uses).

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
    as an `interface Foo {}`, which can be implemented anywhere in your codebase. Within
    a single service, every method must be implemented in the same language.

1.  Link the service implementation through a `rpcimpls` module. This involves
    implementing the `ServiceImplementations` interface, which tells the language
    runtime and, ultimately, `RpcRuntime.gd` which services are implemented in which
    languages:

    ```rust
    impl ServiceImplementations for AppServiceImplementations {
      fn rngService(&self, factory: &GeneratedServiceFactory) -> Option<Box<dyn RngService>> {
        Some(Box::new(RngServiceImpl::new(factory.clone())))
      }
    }
    ```

1.  Call the RPC service. To call an RPC service from a Godot `Node`, construct a
    `GeneratedServiceFactory` with an `RpcClient`; it provides methods to access and
    call each of your defined RPC services.

    To call one service from another service, pass the `GeneratedServiceFactory` into the
    calling service's implementation and use it the same way.

See each language runtime's README with more specific instructions on how to use
`godot-rosetta-rpc` within that language:

| Language | Docs                               |
| -------- | ---------------------------------- |
| Rust     | [README](rust-runtime/README.md)   |
| C#       | [README](csharp-runtime/README.md) |
| Kotlin   | [README](kotlin-runtime/README.md) |

## Extending to a new language

Currently, the following languages are supported:

- Rust
- C#
- Kotlin

Adding support for a new language involves:

1.  Having a standard protobuf code generator for that language (message types only).
1.  Adding a small runtime library implementing `RpcMethodDescriptor`,
    `ServiceRegistry`, `RpcClient`, mirroring `rust-runtime`/`csharp-runtime`/`kotlin-runtime`.
1.  Adding a new `LanguageGenerator` implementation + Askama template set in
    `protoc-gen-rosetta-rpc/templates/<language>/`.
1.  Updating `RpcRuntime.gd` to bootstrap the language runtime, and register the
    services implemented in that language.

No changes are required to `.proto` definitions or any other language's generator
or runtime.

## Limitations / known issues

- **No naming-collision detection.** If a service name collides with an existing
  message/enum name in the same proto package, `protoc` won't catch it — you'll
  instead see a confusing compile error in the generated Rust/Kotlin code, rather
  than a clear `protoc`-level error. (The plugin already has what it needs to detect
  this via `TypeResolver` in `ir.rs`; it just doesn't check yet.)

- **Limited to single-package-per invocation**: the tool fails when used on
  services that span multiple proto packages. (Fixing this would involve keying
  aggregate output per-package.)
