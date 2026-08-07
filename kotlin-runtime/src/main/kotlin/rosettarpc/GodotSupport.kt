package rosettarpc

import godot.api.Node
import godot.core.PackedByteArray
import godot.core.PackedStringArray

/**
 * Thin gdext-facing helper functions consumed by the *generated* `KotlinRuntime` class (emitted per
 * project by `protoc-gen-rosetta-rpc` into that project's `Bootstrap.kt`; never part of this module
 * — see the module README for why). These functions never reference any project-specific generated
 * type, only [ServiceRegistry] and Godot builtins, which is what keeps this module free of any
 * dependency on generated code.
 */
object GodotSupport {
  const val RPC_RUNTIME_AUTOLOAD_PATH = "/root/RpcRuntime"

  /**
   * Builds an `RpcClient` by locating the `RpcRuntime` autoload. This can be used from any scene
   * script to call RPC services.
   *
   * [path] defaults to a common autoload path for the RpcRuntime but can be overridden. (e.g. for
   * tests, or a project that installed `RpcRuntime.gd` under a non-default autoload name).
   */
  fun makeRpcClient(node: Node, rpcRuntimePath: String = RPC_RUNTIME_AUTOLOAD_PATH): RpcClient {
    val rpcRuntime =
        node.getNodeOrNull(rpcRuntimePath)
            ?: error("RpcRuntime autoload not found at $rpcRuntimePath")
    return RpcClient(
        Invoker { serviceId, methodId, requestBytes ->
          val result = rpcRuntime.call("invoke", serviceId, methodId, PackedByteArray(requestBytes))
          (result as PackedByteArray).toByteArray()
        })
  }

  /**
   * NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
   *
   * Builds an `RpcClient` whose outgoing calls are routed through `node`'s parent — which is always
   * `RpcRuntime.gd` in the intended usage, since `RpcRuntime.gd` is what adds the generated
   * `KotlinRuntime` as a child.
   */
  fun bootstrapRpcClient(node: Node): RpcClient {
    return RpcClient(
        Invoker { serviceId, methodId, requestBytes ->
          val parent =
              node.getParent() ?: error("the runtime node must be a child of RpcRuntime.gd")
          val result = parent.call("invoke", serviceId, methodId, PackedByteArray(requestBytes))
          (result as PackedByteArray).toByteArray()
        })
  }

  /**
   * NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
   *
   * Backs the generated `KotlinRuntime`'s `invoke(...)`.
   */
  fun dispatchBytes(
      registry: ServiceRegistry,
      serviceId: String,
      methodId: String,
      requestBytes: PackedByteArray,
  ): PackedByteArray {
    val bytes =
        try {
          Envelope.encodeOk(registry.dispatch(serviceId, methodId, requestBytes.toByteArray()))
        } catch (e: RpcException) {
          Envelope.encodeError(e.code, e.message ?: "")
        } catch (e: com.google.protobuf.InvalidProtocolBufferException) {
          Envelope.encodeError(
              RpcErrorCode.DECODE, e.message ?: "failed to decode request protobuf")
        } catch (e: Exception) {
          Envelope.encodeError(RpcErrorCode.APPLICATION, e.message ?: e.toString())
        }
    return PackedByteArray(bytes)
  }

  /**
   * NOT INTENDED TO BE CALLED DIRECTLY BY CLIENTS!
   *
   * Backs the generated `KotlinRuntime`'s `registeredServiceIds()`.
   */
  fun serviceIds(registry: ServiceRegistry): PackedStringArray {
    val result = PackedStringArray()
    for (id in registry.registeredServiceIds()) {
      result.append(id)
    }
    return result
  }
}
