// GENERATED CODE — do not edit. Source: Rosetta.Example.Clock
namespace Rosetta.Example;

using RosettaRpc;

public class ClockClient : IClock
{
    private readonly RpcClient _rpc;

    public ClockClient(RpcClient rpc)
    {
        _rpc = rpc;
    }


    public Rosetta.Example.CurrentTimeResponse CurrentTime(Rosetta.Example.CurrentTimeRequest request) =>
        _rpc.Call(ClockDescriptors.CURRENT_TIME, request);

}