// GENERATED CODE — do not edit. Aggregates every service compiled in this invocation.
namespace Rosetta.Example;

using RosettaRpc;

// The one hand-written seam: implement `ServiceImplementations` in your own
// project as `public class AppServiceImplementations : ServiceImplementations`
// in namespace `RpcImpls` (mirroring the Rust generator's `crate::rpc_impls`
// / Kotlin's `rpcimpls` convention — a fixed, well-known location so this
// generated file never needs to know your project's own namespace layout).
// Picked up by `CSharpRuntime.Bootstrap()` in CSharpRuntime.cs. Override only
// the services this language actually implements — every method defaults to
// `null`, so a service nobody overrides here simply never gets registered
// (see RpcRuntime.gd's duplicate-registration check for what happens if more
// than one language overrides the same service).
public interface ServiceImplementations
{

    IClock? Clock(GeneratedServiceFactory factory) => null;

    IGameService? GameService(GeneratedServiceFactory factory) => null;

}

public static class Bootstrap
{
    public static void Register(ServiceRegistry registry, RpcClient rpc, ServiceImplementations impls)
    {
        var factory = new GeneratedServiceFactory(rpc);

        if (impls.Clock(factory) is { } ClockImpl)
        {
            registry.Register("Clock", new ClockAdapter(ClockImpl));
        }

        if (impls.GameService(factory) is { } GameServiceImpl)
        {
            registry.Register("GameService", new GameServiceAdapter(GameServiceImpl));
        }

    }
}