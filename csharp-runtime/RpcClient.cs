using Google.Protobuf;

namespace RosettaRpc;

/// <summary>
/// Generated clients call through an <see cref="RpcClient"/> instead of
/// talking to any transport directly. <see cref="Invoker"/> is the entire
/// seam between "pure C# logic" and "however calls actually leave this
/// process" — in production it forwards into <c>RpcRuntime.gd</c>, but in a
/// test it can call a local <see cref="ServiceRegistry.Dispatch"/> directly,
/// with no Godot involved at all.
/// </summary>
public delegate byte[] Invoker(string serviceId, string methodId, byte[] requestBytes);

public sealed class RpcClient
{
    private readonly Invoker _invoker;

    public RpcClient(Invoker invoker) => _invoker = invoker;

    /// <exception cref="RpcException">
    /// Thrown if the call failed — at the transport/framework level (unknown
    /// service/method, decode failure) or because the service implementation
    /// itself failed.
    /// </exception>
    public TResp Call<TReq, TResp>(RpcMethodDescriptor<TReq, TResp> descriptor, TReq request)
        where TReq : IMessage<TReq>
        where TResp : IMessage<TResp>
    {
        var responseBytes = _invoker(descriptor.ServiceId, descriptor.MethodId, request.ToByteArray());
        var payload = Envelope.Decode(responseBytes);
        try
        {
            return descriptor.ResponseParser.ParseFrom(payload);
        }
        catch (InvalidProtocolBufferException e)
        {
            throw new RpcException(RpcErrorCode.Decode, e.Message);
        }
    }
}
