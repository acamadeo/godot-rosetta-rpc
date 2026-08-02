// GENERATED CODE — do not edit. Found by RpcRuntime.gd via a directory scan.
// This is a distinct file from Boostrap.cs, since Godot resolves a C# script's
// instantiable type by matching the file name.
namespace Rosetta.Example;

using Godot;
using RosettaRpc;

public partial class CSharpRuntime : Node
{
    private readonly ServiceRegistry _registry = new();

    public void Bootstrap()
    {
        var rpc = GodotSupport.MakeRpcClient(this);
        // Namespace-qualified: an unqualified `Bootstrap` here would resolve
        // to this enclosing method itself (C# simple-name lookup checks the
        // containing type's members before namespace-level types), not the
        // `Bootstrap` static class in Bootstrap.cs.
        Rosetta.Example.Bootstrap.Register(_registry, rpc, new RpcImpls.AppServiceImplementations());
    }

    public byte[] Invoke(string serviceId, string methodId, byte[] requestBytes) =>
        GodotSupport.DispatchBytes(_registry, serviceId, methodId, requestBytes);

    public string[] RegisteredServiceIds() => GodotSupport.ServiceIds(_registry);
}