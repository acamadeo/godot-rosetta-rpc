# godot-rosetta-rpc (C# runtime)

The C# runtime for the [godot-rosetta-rpc](../README.md) cross-language RPC
framework: `RpcMethodDescriptor`, `ServiceRegistry`, `RpcClient`, plus a
`GodotSupport` static class to plug in the rest of the library with Godot.
This library never depends on any project-specific generated code.

## Installing

```xml
<ItemGroup>
  <PackageReference Include="GodotRosettaRpc" Version="0.1.0" />
</ItemGroup>
```

### Project setup

Your project also needs:

- **A .NET-enabled Godot editor/export templates already set up.** C# support is built into Godot itself, but since it doesn't yet
  support GDExtension, it requires a Mono-specific binary.

- **`protoc`'s generated message + RPC glue sources on the compile path.**
  An SDK-style `.csproj` can glob-include every `.cs` file under the project
  directory by default. Simply point `protoc --csharp_out=...` and
  `--rosetta-rpc_out=lang=csharp:...` somewhere under your project root and
  no further wiring is needed. See [example/generate.py](../example/generate.py)
  for a working configuration.

## Implementing an RPC service

Each `service Foo {}` in your `.proto` file generates a C# `interface IFoo`
with one method per RPC. Implement it anywhere in
your codebase — the generated interface doesn't assume any particular
namespace or location:

```csharp
public class BuzzerImpl : IBuzzer
{
    private readonly GeneratedServiceFactory _factory;

    public BuzzerImpl(GeneratedServiceFactory factory) => _factory = factory;

    public PingResponse Ping(PingRequest request)
    {
        var clock = _factory.Clock();
        var currentTime = clock.CurrentTime(new CurrentTimeRequest());
        return new PingResponse { Message = $"pong: {request.Message} @ {currentTime.Millis}ms" };
    }
}
```

## Register an RPC service

Every service implemented in C# must be linked through a single
`public class AppServiceImplementations` in namespace `RpcImpls`,
implementing the generated `ServiceImplementations` interface. That fixed,
well-known location is what lets the generated `Bootstrap.cs` find your
implementations without depending on your project's own namespace layout:

```csharp
namespace RpcImpls;

public class AppServiceImplementations : ServiceImplementations
{
    public IBuzzer? Buzzer(GeneratedServiceFactory factory) => new BuzzerImpl(factory);
}
```

Only override the services implemented in C# — every other method defaults
to returning `null`, meaning "not implemented in this language."

## Call an RPC service

From any Godot `Node`, build an `RpcClient` with
`GodotSupport.MakeRpcClient(this)`, wrap it in the generated
`GeneratedServiceFactory`, then call through the factory:

```csharp
[GlobalClass]
public partial class MyNode : Node2D
{
    public override void _Ready()
    {
        var services = new GeneratedServiceFactory(GodotSupport.MakeRpcClient(this));
        var response = services.Buzzer().Ping(new PingRequest { Message = "hello" });
        GD.Print(response.Message);
    }
}
```

To call one service from another's implementation, reuse the
`GeneratedServiceFactory` passed into your constructor (see above) instead of
building a new one.
