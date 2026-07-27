// GENERATED CODE — do not edit. Source: rosetta.example.Clock
package rosetta.example

import rosettarpc.Adapter

class ClockAdapter(private val implementation: Clock) : Adapter {
    override fun invoke(methodId: String, requestBytes: ByteArray): ByteArray = when (methodId) {

        "CurrentTime" -> implementation.currentTime(rosetta.example.CurrentTimeRequest.parseFrom(requestBytes)).toByteArray()

        else -> throw IllegalArgumentException("unknown method: $methodId")
    }
}