namespace RosettaRpc;

/// <summary>
/// Stable, wire-level discriminant for an <see cref="RpcException"/>. Must
/// stay in sync with <c>RpcError</c> (Rust) and <c>RpcErrorCode</c> (Kotlin).
/// </summary>
public enum RpcErrorCode
{
    UnknownService = 0,
    UnknownMethod = 1,
    Decode = 2,

    /// <summary>
    /// The service implementation itself failed: either it deliberately
    /// threw, or an otherwise-uncaught exception was caught on its behalf by
    /// <c>GodotSupport.DispatchBytes</c>.
    /// </summary>
    Application = 3,
}
