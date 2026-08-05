using Google.Protobuf;

namespace RosettaRpc;

/// <summary>
/// Identifies one RPC method and carries the typed parsers needed to
/// encode/decode it. Const-constructible and never references any
/// project-specific generated type.
/// </summary>
public sealed class RpcMethodDescriptor<TReq, TResp>
    where TReq : IMessage<TReq>
    where TResp : IMessage<TResp>
{
    public string ServiceId { get; }
    public string MethodId { get; }
    public MessageParser<TReq> RequestParser { get; }
    public MessageParser<TResp> ResponseParser { get; }

    public RpcMethodDescriptor(
        string serviceId,
        string methodId,
        MessageParser<TReq> requestParser,
        MessageParser<TResp> responseParser)
    {
        ServiceId = serviceId;
        MethodId = methodId;
        RequestParser = requestParser;
        ResponseParser = responseParser;
    }
}
