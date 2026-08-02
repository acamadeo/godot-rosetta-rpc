// GENERATED CODE — do not edit. Source: Rosetta.Example.Clock
namespace Rosetta.Example;

using System;
using Google.Protobuf;
using RosettaRpc;

public class ClockAdapter : IAdapter
{
    private readonly IClock _implementation;

    public ClockAdapter(IClock implementation)
    {
        _implementation = implementation;
    }

    public byte[] Invoke(string methodId, byte[] requestBytes) => methodId switch
    {

        "CurrentTime" => _implementation.CurrentTime(Rosetta.Example.CurrentTimeRequest.Parser.ParseFrom(requestBytes)).ToByteArray(),

        _ => throw new ArgumentException($"unknown method: {methodId}"),
    };
}