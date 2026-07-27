use std::rc::Rc;

use prost::Message;

use crate::descriptor::RpcMethodDescriptor;

/// Generated clients call through an `RpcClient` instead of talking to any
/// transport directly. The `invoke` closure is the entire seam between "pure
/// Rust logic" and "however calls actually leave this process" — in
/// production it forwards into `RpcRuntime.gd`, but in a test it can just
/// call a local `ServiceRegistry::dispatch` directly, with no Godot or FFI
/// involved at all.
///
// Deliberately not `Send + Sync`: RPC calls in this framework are
// synchronous, in-process, same-thread calls (no networking, no
// concurrency), and the closure often captures a `Gd<Node>` handle — Godot
// objects are inherently single-threaded and never `Send`/`Sync`.
#[derive(Clone)]
pub struct RpcClient {
    invoke: Rc<dyn Fn(&str, &str, &[u8]) -> Vec<u8>>,
}

impl RpcClient {
    pub fn new(invoke: impl Fn(&str, &str, &[u8]) -> Vec<u8> + 'static) -> Self {
        Self {
            invoke: Rc::new(invoke),
        }
    }

    pub fn call<Req, Resp>(&self, descriptor: &RpcMethodDescriptor<Req, Resp>, request: Req) -> Resp
    where
        Req: Message,
        Resp: Message + Default,
    {
        let request_bytes = request.encode_to_vec();
        let response_bytes =
            (self.invoke)(descriptor.service_id, descriptor.method_id, &request_bytes);
        Resp::decode(response_bytes.as_slice()).expect("invalid response protobuf")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::ServiceRegistry;

    #[test]
    fn round_trips_through_a_local_registry() {
        // Reuses the pure-Rust smoke-test shape from registry.rs's tests,
        // but through the RpcClient/descriptor seam instead of calling
        // `dispatch` directly.
        let mut registry = ServiceRegistry::new();
        registry.register(
            "Echoer",
            Box::new(EchoAdapter) as Box<dyn crate::registry::ErasedAdapter>,
        );

        let client = RpcClient::new(move |service_id, method_id, bytes| {
            registry.dispatch(service_id, method_id, bytes).unwrap()
        });

        let descriptor = RpcMethodDescriptor::<prost_types::Duration, prost_types::Duration>::new(
            "Echoer", "Echo",
        );
        let request = prost_types::Duration {
            seconds: 42,
            nanos: 7,
        };
        let response = client.call(&descriptor, request.clone());
        assert_eq!(response, request);
    }

    struct EchoAdapter;
    impl crate::registry::ErasedAdapter for EchoAdapter {
        fn invoke(
            &self,
            _method_id: &str,
            request_bytes: &[u8],
        ) -> Result<Vec<u8>, crate::error::RpcError> {
            Ok(request_bytes.to_vec())
        }
    }
}
