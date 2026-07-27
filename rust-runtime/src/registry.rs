use std::collections::HashMap;

use crate::error::RpcError;

/// A generated adapter: decodes a request, invokes a concrete service
/// implementation, encodes the response. Type-erased (`&str` method id, raw
/// bytes in and out) because a [`ServiceRegistry`] must be able to dispatch
/// to any service without being generic over its request/response types.
pub trait ErasedAdapter {
    fn invoke(&self, method_id: &str, request_bytes: &[u8]) -> Result<Vec<u8>, RpcError>;
}

/// Holds every locally-implemented service's adapter, keyed by service id,
/// and dispatches incoming (service_id, method_id, bytes) calls to the right
/// one. Registered by a project's generated `Bootstrap::register(...)`.
///
/// Not `Send + Sync`: adapters commonly wrap implementations that hold a
/// `Gd<T>` handle, and Godot objects are inherently single-threaded.
#[derive(Default)]
pub struct ServiceRegistry {
    adapters: HashMap<String, Box<dyn ErasedAdapter>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, service_id: impl Into<String>, adapter: Box<dyn ErasedAdapter>) {
        self.adapters.insert(service_id.into(), adapter);
    }

    pub fn dispatch(
        &self,
        service_id: &str,
        method_id: &str,
        request_bytes: &[u8],
    ) -> Result<Vec<u8>, RpcError> {
        self.adapters
            .get(service_id)
            .ok_or(RpcError::UnknownService)?
            .invoke(method_id, request_bytes)
    }

    pub fn registered_service_ids(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoAdapter;
    impl ErasedAdapter for EchoAdapter {
        fn invoke(&self, method_id: &str, request_bytes: &[u8]) -> Result<Vec<u8>, RpcError> {
            match method_id {
                "Echo" => Ok(request_bytes.to_vec()),
                _ => Err(RpcError::UnknownMethod),
            }
        }
    }

    #[test]
    fn dispatches_to_registered_service() {
        let mut registry = ServiceRegistry::new();
        registry.register("Echoer", Box::new(EchoAdapter));

        let result = registry.dispatch("Echoer", "Echo", b"hello").unwrap();
        assert_eq!(result, b"hello");
    }

    #[test]
    fn unknown_service_errors() {
        let registry = ServiceRegistry::new();
        assert_eq!(
            registry.dispatch("Nope", "Echo", b""),
            Err(RpcError::UnknownService)
        );
    }

    #[test]
    fn unknown_method_errors() {
        let mut registry = ServiceRegistry::new();
        registry.register("Echoer", Box::new(EchoAdapter));
        assert_eq!(
            registry.dispatch("Echoer", "Nope", b""),
            Err(RpcError::UnknownMethod)
        );
    }

    #[test]
    fn lists_registered_service_ids() {
        let mut registry = ServiceRegistry::new();
        registry.register("Echoer", Box::new(EchoAdapter));
        assert_eq!(
            registry.registered_service_ids(),
            vec!["Echoer".to_string()]
        );
    }
}
