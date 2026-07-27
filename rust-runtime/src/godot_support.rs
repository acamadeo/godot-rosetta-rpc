//! Thin gdext-facing helper functions consumed by the *generated*
//! `RustRuntime` GodotClass (emitted per project by `protoc-gen-rosetta-rpc`
//! into that project's `bootstrap.rs`; never part of this crate — see the
//! crate README for why). These functions never reference any
//! project-specific generated type, only [`ServiceRegistry`] and Godot
//! builtins, which is what keeps this crate free of any dependency on
//! generated code.

use godot::builtin::{GString, PackedByteArray, PackedStringArray, Variant};
use godot::classes::Node;
use godot::obj::Gd;

use crate::client::RpcClient;
use crate::registry::ServiceRegistry;

pub const RPC_RUNTIME_AUTOLOAD_PATH: &str = "/root/RpcRuntime";

/// Builds an `RpcClient` by locating the `RpcRuntime` autoload. This can be used from any scene script to call RPC services.
pub fn make_rpc_client(node: Gd<Node>, path: Option<&str>) -> RpcClient {
    let path = path.unwrap_or(RPC_RUNTIME_AUTOLOAD_PATH).to_string();
    RpcClient::new(move |service_id, method_id, request_bytes| {
        let mut rpc_runtime = node
            .get_node_or_null(&path)
            .unwrap_or_else(|| panic!("RpcRuntime autoload not found at {path}"));
        let args = [
            Variant::from(service_id),
            Variant::from(method_id),
            Variant::from(PackedByteArray::from(request_bytes)),
        ];
        let result = rpc_runtime.call("invoke", &args);
        result
            .try_to::<PackedByteArray>()
            .expect("RpcRuntime.invoke did not return a PackedByteArray")
            .to_vec()
    })
}

/// NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
///
/// Builds an `RpcClient` whose outgoing calls are routed through `node`'s
/// parent — which is always `RpcRuntime.gd` in the intended usage, since
/// `RpcRuntime.gd` is what adds the generated `RustRuntime` as a child.
pub fn bootstrap_rpc_client(node: Gd<Node>) -> RpcClient {
    RpcClient::new(move |service_id, method_id, request_bytes| {
        let mut parent = node
            .get_parent()
            .expect("the runtime node must be a child of RpcRuntime.gd");
        let args = [
            Variant::from(service_id),
            Variant::from(method_id),
            Variant::from(PackedByteArray::from(request_bytes)),
        ];
        let result = parent.call("invoke", &args);
        result
            .try_to::<PackedByteArray>()
            .expect("RpcRuntime.invoke did not return a PackedByteArray")
            .to_vec()
    })
}

/// NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
///
/// Backs the generated `RustRuntime`'s `#[func] invoke(...)`.
pub fn dispatch_bytes(
    registry: &ServiceRegistry,
    service_id: GString,
    method_id: GString,
    request_bytes: PackedByteArray,
) -> PackedByteArray {
    let bytes = registry
        .dispatch(
            &service_id.to_string(),
            &method_id.to_string(),
            request_bytes.to_vec().as_slice(),
        )
        .unwrap_or_else(|e| panic!("rpc dispatch error: {e}"));
    PackedByteArray::from(bytes.as_slice())
}

/// NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
///
/// Backs the generated `RustRuntime`'s `#[func] registered_service_ids()`.
pub fn service_ids(registry: &ServiceRegistry) -> PackedStringArray {
    registry
        .registered_service_ids()
        .into_iter()
        .map(|id| GString::from(id.as_str()))
        .collect()
}
