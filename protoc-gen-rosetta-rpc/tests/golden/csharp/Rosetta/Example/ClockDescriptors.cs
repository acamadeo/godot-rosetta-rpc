// GENERATED CODE — do not edit. Source: Rosetta.Example.Clock
namespace Rosetta.Example;

using RosettaRpc;

public static class ClockDescriptors
{

    public static readonly RpcMethodDescriptor<Rosetta.Example.CurrentTimeRequest, Rosetta.Example.CurrentTimeResponse> CURRENT_TIME =
        new("Clock", "CurrentTime", Rosetta.Example.CurrentTimeRequest.Parser, Rosetta.Example.CurrentTimeResponse.Parser);

}