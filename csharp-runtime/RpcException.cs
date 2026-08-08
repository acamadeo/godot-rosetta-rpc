namespace RosettaRpc;

/// <summary>
/// Thrown by <see cref="RpcClient.Call{TReq,TResp}"/> when a call fails —
/// either at the transport/framework level (unknown service/method, decode
/// failure) or because the service implementation itself failed. Callers
/// distinguish these with <see cref="Code"/>.
/// </summary>
public sealed class RpcException : Exception
{
    public RpcErrorCode Code { get; }

    public RpcException(RpcErrorCode code, string message) : base(message)
    {
        Code = code;
    }
}
