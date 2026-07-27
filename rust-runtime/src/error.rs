use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RpcError {
    UnknownService,
    UnknownMethod,
    Decode,
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcError::UnknownService => write!(f, "no service registered for that service id"),
            RpcError::UnknownMethod => write!(f, "service has no such method"),
            RpcError::Decode => write!(f, "failed to decode request protobuf"),
        }
    }
}

impl std::error::Error for RpcError {}
