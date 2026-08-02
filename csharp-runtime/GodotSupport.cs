using Godot;

namespace RosettaRpc;

/// <summary>
/// Thin Godot-facing helper functions consumed by the *generated*
/// <c>CSharpRuntime</c> class (emitted per project by
/// <c>protoc-gen-rosetta-rpc</c> into that project's <c>Bootstrap.cs</c>).
/// It can also be used to create RPC clients from Godot nodes.
/// </summary>
public static class GodotSupport
{
    public const string RpcRuntimeAutoloadPath = "/root/RpcRuntime";

    /// <summary>
    /// Builds an <see cref="RpcClient"/> by locating the <c>RpcRuntime</c>
    /// autoload. Can be used from any scene script to call RPC services.
    /// <paramref name="rpcRuntimePath"/> defaults to the common autoload
    /// path but can be overridden (e.g. for tests, or a project that
    /// installed <c>RpcRuntime.gd</c> under a non-default autoload name).
    /// </summary>
    public static RpcClient MakeRpcClient(Node node, string rpcRuntimePath = RpcRuntimeAutoloadPath)
    {
        var rpcRuntime = node.GetNodeOrNull(rpcRuntimePath)
            ?? throw new InvalidOperationException($"RpcRuntime autoload not found at {rpcRuntimePath}");
        return new RpcClient((serviceId, methodId, requestBytes) =>
            (byte[])rpcRuntime.Call("invoke", serviceId, methodId, requestBytes));
    }

    /// <summary>
    /// NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS! Backs the generated
    /// <c>CSharpRuntime</c>'s <c>Invoke(...)</c>.
    /// </summary>
    public static byte[] DispatchBytes(ServiceRegistry registry, string serviceId, string methodId, byte[] requestBytes) =>
        registry.Dispatch(serviceId, methodId, requestBytes);

    /// <summary>
    /// NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS! Backs the generated
    /// <c>CSharpRuntime</c>'s <c>RegisteredServiceIds()</c>.
    /// </summary>
    public static string[] ServiceIds(ServiceRegistry registry) => registry.RegisteredServiceIds().ToArray();
}
