use std::marker::PhantomData;

/// Identifies one RPC method: a service id, a method id, and the
/// request/response types (carried only as a `PhantomData` — descriptors are
/// zero-cost, `const`-constructible identifiers, not runtime values).
#[derive(Debug)]
pub struct RpcMethodDescriptor<Req, Resp> {
    pub service_id: &'static str,
    pub method_id: &'static str,
    _marker: PhantomData<fn(Req) -> Resp>,
}

impl<Req, Resp> RpcMethodDescriptor<Req, Resp> {
    pub const fn new(service_id: &'static str, method_id: &'static str) -> Self {
        Self {
            service_id,
            method_id,
            _marker: PhantomData,
        }
    }
}

// Manual impls instead of `#[derive(Clone, Copy)]`: the derive would require
// `Req: Clone`/`Req: Copy` bounds on the struct even though `Req`/`Resp` only
// ever appear inside `PhantomData`, which is `Clone`/`Copy` regardless of its
// parameter.
impl<Req, Resp> Clone for RpcMethodDescriptor<Req, Resp> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Req, Resp> Copy for RpcMethodDescriptor<Req, Resp> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_ids() {
        let descriptor = RpcMethodDescriptor::<(), ()>::new("Clock", "CurrentTime");
        assert_eq!(descriptor.service_id, "Clock");
        assert_eq!(descriptor.method_id, "CurrentTime");
    }
}
