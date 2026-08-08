// GENERATED CODE — do not edit. Source: Rosetta.Example.GameService
namespace Rosetta.Example;

using Google.Protobuf;
using RosettaRpc;

public class GameServiceAdapter : IAdapter
{
    private readonly IGameService _implementation;

    public GameServiceAdapter(IGameService implementation)
    {
        _implementation = implementation;
    }

    public byte[] Invoke(string methodId, byte[] requestBytes) => methodId switch
    {

        "Ping" => _implementation.Ping(Rosetta.Example.PingRequest.Parser.ParseFrom(requestBytes)).ToByteArray(),

        _ => throw new RpcException(RpcErrorCode.UnknownMethod, $"unknown method: {methodId}"),
    };
}