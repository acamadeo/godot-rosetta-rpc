package rosettarpc

/**
 * A generated adapter: decodes a request, invokes a concrete service implementation, encodes the
 * response.
 */
fun interface Adapter {
  fun invoke(methodId: String, requestBytes: ByteArray): ByteArray
}

/**
 * Holds every locally-implemented service's adapter, keyed by service id, and dispatches incoming
 * (serviceId, methodId, bytes) calls to the right one. Registered by a project's generated
 * `Bootstrap.register(...)`.
 */
class ServiceRegistry {
  private val services = mutableMapOf<String, Adapter>()

  fun register(serviceId: String, adapter: Adapter) {
    services[serviceId] = adapter
  }

  fun dispatch(serviceId: String, methodId: String, requestBytes: ByteArray): ByteArray {
    val adapter =
        services[serviceId]
            ?: throw IllegalArgumentException("no service registered for service id: $serviceId")
    return adapter.invoke(methodId, requestBytes)
  }

  fun registeredServiceIds(): List<String> = services.keys.toList()
}
