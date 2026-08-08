use std::collections::HashMap;
use std::panic::{self, AssertUnwindSafe};

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
        let adapter = self
            .adapters
            .get(service_id)
            .ok_or(RpcError::UnknownService)?;

        // A panic inside a service implementation must never unwind across
        // the Godot Node.call() boundary. Instead, convert it into a normal
        // `RpcError::Application`, same as an implementation that
        // deliberately returns `Err(...)`.
        panic::catch_unwind(AssertUnwindSafe(|| {
            adapter.invoke(method_id, request_bytes)
        }))
        .unwrap_or_else(|payload| Err(RpcError::Application(panic_message(payload))))
    }

    pub fn registered_service_ids(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

/// Extracts a human-readable message from a `std::panic::catch_unwind`
/// payload, covering the two payload shapes the standard panic hook
/// produces (`&str`` and `String`).
fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        message.to_string()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "service implementation panicked".to_string()
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
                "Panic" => panic!("simulated panic"),
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
    fn panic_in_adapter_becomes_application_error() {
        let mut registry = ServiceRegistry::new();
        registry.register("Echoer", Box::new(EchoAdapter));

        // Silence the default panic hook's stderr output for this
        // deliberately-triggered panic.
        let previous_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        let result = registry.dispatch("Echoer", "Panic", b"");
        panic::set_hook(previous_hook);

        assert_eq!(
            result,
            Err(RpcError::Application("simulated panic".to_string()))
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
