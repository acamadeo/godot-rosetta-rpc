// GENERATED CODE — do not edit. Aggregates every service compiled in this invocation.
namespace Rosetta.Example;

using RosettaRpc;

public class GeneratedServiceFactory
{
    private readonly RpcClient _rpc;

    public GeneratedServiceFactory(RpcClient rpc)
    {
        _rpc = rpc;
    }


    public ClockClient Clock() => new ClockClient(_rpc);

    public GameServiceClient GameService() => new GameServiceClient(_rpc);

}