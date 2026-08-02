// GENERATED CODE — do not edit. Source: Rosetta.Example.GameService
namespace Rosetta.Example;

using RosettaRpc;

public static class GameServiceDescriptors
{

    public static readonly RpcMethodDescriptor<Rosetta.Example.PingRequest, Rosetta.Example.PingResponse> PING =
        new("GameService", "Ping", Rosetta.Example.PingRequest.Parser, Rosetta.Example.PingResponse.Parser);

}