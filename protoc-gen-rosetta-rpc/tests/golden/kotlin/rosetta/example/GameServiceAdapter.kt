// GENERATED CODE — do not edit. Source: rosetta.example.GameService
package rosetta.example

import rosettarpc.Adapter
import rosettarpc.RpcErrorCode
import rosettarpc.RpcException

class GameServiceAdapter(private val implementation: GameService) : Adapter {
    override fun invoke(methodId: String, requestBytes: ByteArray): ByteArray = when (methodId) {

        "Ping" -> implementation.ping(rosetta.example.PingRequest.parseFrom(requestBytes)).toByteArray()

        else -> throw RpcException(RpcErrorCode.UNKNOWN_METHOD, "unknown method: $methodId")
    }
}