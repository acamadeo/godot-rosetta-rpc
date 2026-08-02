// GENERATED CODE — do not edit. Source: Rosetta.Example.GameService
namespace Rosetta.Example;

using RosettaRpc;

public class GameServiceClient : IGameService
{
    private readonly RpcClient _rpc;

    public GameServiceClient(RpcClient rpc)
    {
        _rpc = rpc;
    }


    public Rosetta.Example.PingResponse Ping(Rosetta.Example.PingRequest request) =>
        _rpc.Call(GameServiceDescriptors.PING, request);

}