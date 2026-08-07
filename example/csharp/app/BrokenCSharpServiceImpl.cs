using System;
using Rosetta.Example;

namespace Example.App;

/// <summary>
/// Deliberately fails every call. For testing error propagation.
/// </summary>
public class BrokenCSharpServiceImpl : IBrokenCSharp
{
    public FailResponse Fail(FailRequest request) =>
        throw new InvalidOperationException("BrokenCSharp: deliberate failure");
}
