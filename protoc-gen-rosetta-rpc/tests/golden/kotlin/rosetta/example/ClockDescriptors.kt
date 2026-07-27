// GENERATED CODE — do not edit. Source: rosetta.example.Clock
package rosetta.example

import rosettarpc.RpcMethodDescriptor

object ClockDescriptors {

    val CURRENT_TIME = RpcMethodDescriptor(
        serviceId = "Clock",
        methodId = "CurrentTime",
        requestParser = rosetta.example.CurrentTimeRequest.parser(),
        responseParser = rosetta.example.CurrentTimeResponse.parser(),
    )

}