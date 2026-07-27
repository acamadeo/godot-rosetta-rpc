// GENERATED CODE — do not edit. Aggregates every service compiled in this invocation.
package rosetta.example

import rosettarpc.RpcClient
import rosettarpc.ServiceRegistry
import rosettarpc.GodotSupport.bootstrapRpcClient
import rosettarpc.GodotSupport.dispatchBytes
import rosettarpc.GodotSupport.serviceIds

// The one hand-written seam: implement `ServiceImplementations` in your own
// project as `object AppServiceImplementations` in a package named
// `rpcimpls` (mirroring the Rust generator's `crate::rpc_impls` convention —
// a fixed, well-known location so this generated file never needs to know
// your project's own package layout). Picked up by `KotlinRuntime.bootstrap()`
// below. Override only the services this language actually implements —
// every method defaults to `null`, so a service nobody overrides here simply
// never gets registered (see RpcRuntime.gd's duplicate-registration check
// for what happens if more than one language overrides the same service).
interface ServiceImplementations {

    fun clock(factory: GeneratedServiceFactory): Clock? = null

    fun gameService(factory: GeneratedServiceFactory): GameService? = null

}

object Bootstrap {
    fun register(
        registry: ServiceRegistry,
        rpc: RpcClient,
        impls: ServiceImplementations,
    ) {
        val factory = GeneratedServiceFactory(rpc)

        impls.clock(factory)?.let {
            registry.register("Clock", ClockAdapter(it))
        }

        impls.gameService(factory)?.let {
            registry.register("GameService", GameServiceAdapter(it))
        }

    }
}

@godot.annotation.RegisterClass
class KotlinRuntime : godot.api.Node() {
    private val registry = ServiceRegistry()

    @godot.annotation.RegisterFunction
    fun bootstrap() {
        val rpc = bootstrapRpcClient(this)
        Bootstrap.register(registry, rpc, rpcimpls.AppServiceImplementations)
    }

    @godot.annotation.RegisterFunction
    fun invoke(serviceId: String, methodId: String, requestBytes: godot.core.PackedByteArray): godot.core.PackedByteArray =
        dispatchBytes(registry, serviceId, methodId, requestBytes)

    @godot.annotation.RegisterFunction
    fun registeredServiceIds(): godot.core.PackedStringArray =
        serviceIds(registry)
}