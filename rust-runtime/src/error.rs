use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RpcError {
    UnknownService,
    UnknownMethod,
    Decode,
    /// The service implementation itself failed: either it deliberately
    /// returned `Err(...)`, or an uncaught panic was caught on its behalf by
    /// `ServiceRegistry::dispatch`.
    Application(String),
}

impl RpcError {
    /// A stable, wire-level discriminant for this variant. Used by the
    /// envelope encoding — must stay in sync across all language runtimes.
    pub fn code(&self) -> i32 {
        match self {
            RpcError::UnknownService => 0,
            RpcError::UnknownMethod => 1,
            RpcError::Decode => 2,
            RpcError::Application(_) => 3,
        }
    }

    pub fn message(&self) -> String {
        self.to_string()
    }

    /// Reconstructs an `RpcError` from a wire-level code and message, as
    /// decoded off the envelope. Unrecognized codes are treated as
    /// `Application` errors so a version-skewed service's error still
    /// surfaces as an error rather than being silently dropped.
    pub fn from_code(code: i32, message: String) -> Self {
        match code {
            0 => RpcError::UnknownService,
            1 => RpcError::UnknownMethod,
            2 => RpcError::Decode,
            _ => RpcError::Application(message),
        }
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RpcError::UnknownService => write!(f, "no service registered for that service id"),
            RpcError::UnknownMethod => write!(f, "service has no such method"),
            RpcError::Decode => write!(f, "failed to decode request protobuf"),
            RpcError::Application(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for RpcError {}

/// The error type generated service trait methods return. This is
/// automatically converted to `RpcError::Application` over the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceErr {
    message: String,
}

impl ServiceErr {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    /// Shorthand method to generate an `Err`` result with a given message.
    pub fn msg<T>(message: impl Into<String>) -> Result<T, Self> {
        Err(Self::new(message))
    }
}

impl fmt::Display for ServiceErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Lets `?` convert *any* std error — `io::Error`, `SystemTimeError`, a
/// cross-service call's `RpcError`, etc. — directly into `ServiceErr`.
impl<E: std::error::Error> From<E> for ServiceErr {
    fn from(error: E) -> Self {
        ServiceErr {
            message: error.to_string(),
        }
    }
}

/// Generated adapters use this (via `?`) to turn a service implementation's
/// returned `ServiceErr` into the wire error.
impl From<ServiceErr> for RpcError {
    fn from(error: ServiceErr) -> Self {
        RpcError::Application(error.message)
    }
}
