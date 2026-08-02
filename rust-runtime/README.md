# godot-rosetta-rpc (Rust runtime)

The Rust runtime for the [godot-rosetta-rpc](../README.md) cross-language
RPC framework: `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient`, plus a
`godot_support` module to plug the rest of the library into Godot. This
crate never depends on any project-specific generated code.

## Installing

```toml
[dependencies]
godot-rosetta-rpc = "0.1"
```

### Project setup

Your crate also needs:

- **gdext already set up.** `godot-rosetta-rpc` depends on `godot` (gdext)
  like any other Rust dependency, but your project still needs the usual
  gdext extension scaffolding: `crate-type = ["cdylib"]`, an
  `ExtensionLibrary` impl, and a `.gdextension` file pointing at the built
  library.

- **Generated glue included at the crate root.** `protoc-gen-rosetta-rpc`
  writes loose `.rs` files, not a standalone crate — `include!` its
  generated module tree into your crate root. Pass `message_crate=<crate>`
  to `--rosetta-rpc_out` so the generated code can reference the plain
  message types generated separately (e.g. by `protoc-gen-prost`):

  ```rust
  // crate root (e.g. src/lib.rs)
  include!("generated/mod_tree.rs");
  ```

See [example/generate.py](../example/generate.py) and
[example/rust/game](../example/rust/game) for a working setup.

## Implementing an RPC service

Each `service Foo {}` in your `.proto` file generates a Rust `trait Foo`
with one method per RPC. Implement it anywhere in your crate — the
generated trait doesn't assume any particular module:

```rust
struct ProfilerImpl {
    factory: GeneratedServiceFactory,
}

impl Profiler for ProfilerImpl {
    fn profile(&self, _request: ProfileRequest) -> ProfileResponse {
        let millis = self.factory.clock().current_time(CurrentTimeRequest {}).millis;
        ProfileResponse { message: format!("clock read {millis}ms") }
    }
}
```

## Register an RPC service

Every service implemented in Rust must be linked through a single
`AppServiceImplementations` struct in module `rpc_impls`, implementing the
generated `ServiceImplementations` trait. That fixed, well-known location is
what lets the generated `bootstrap.rs` find your implementations without
depending on your crate's own module layout:

```rust
// crate::rpc_impls
pub struct AppServiceImplementations;

impl ServiceImplementations for AppServiceImplementations {
    fn profiler(&self, factory: &GeneratedServiceFactory) -> Option<Box<dyn Profiler>> {
        Some(Box::new(ProfilerImpl::new(factory.clone())))
    }
}
```

Only override the services implemented in Rust — every other method defaults
to returning `None`, meaning "not implemented in this language."

## Call an RPC service

From any gdext `Node`, build an `RpcClient` with
`godot_support::make_rpc_client(node, None)`, wrap it in the generated
`GeneratedServiceFactory`, then call through the factory:

```rust
#[derive(GodotClass)]
#[class(base=Node, init)]
struct MyNode {
    base: Base<Node>,
}

#[godot_api]
impl INode for MyNode {
    fn ready(&mut self) {
        let node = self.to_gd().upcast::<Node>();
        let rpc = godot_rosetta_rpc::godot_support::make_rpc_client(node, None);
        let services = GeneratedServiceFactory::new(rpc);
        let response = services.profiler().profile(ProfileRequest {});
        godot_print!("{}", response.message);
    }
}
```

To call one service from another's implementation, reuse the
`GeneratedServiceFactory` passed into your constructor (see above) instead of
building a new one.
