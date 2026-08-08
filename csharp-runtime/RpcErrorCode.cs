namespace RosettaRpc;

/// <summary>
/// Wire-level discriminant for an <see cref="RpcException"/>. Must
/// stay in sync across all language runtimes.
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
