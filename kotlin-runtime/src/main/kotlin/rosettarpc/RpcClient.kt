package rosettarpc

import com.google.protobuf.Message

/**
 * Generated clients call through an `RpcClient` instead of talking to any transport directly. The
 * `Invoker` is the entire seam between "pure Kotlin logic" and "however calls actually leave this
 * process" — in production it forwards into `RpcRuntime.gd`, but in a test it can call a local
 * `ServiceRegistry.dispatch` directly, with no Godot involved at all.
 */
fun interface Invoker {
  fun invoke(serviceId: String, methodId: String, requestBytes: ByteArray): ByteArray
}

class RpcClient(private val invoker: Invoker) {
  fun <Req : Message, Resp : Message> call(
      descriptor: RpcMethodDescriptor<Req, Resp>,
      request: Req
  ): Resp {
    val responseBytes =
        invoker.invoke(descriptor.serviceId, descriptor.methodId, request.toByteArray())
    return descriptor.responseParser.parseFrom(responseBytes)
  }
}
