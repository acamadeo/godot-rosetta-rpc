// GENERATED CODE — do not edit. Source: rosetta.example.GameService
package rosetta.example

import rosettarpc.RpcMethodDescriptor

object GameServiceDescriptors {

    val PING = RpcMethodDescriptor(
        serviceId = "GameService",
        methodId = "Ping",
        requestParser = rosetta.example.PingRequest.parser(),
        responseParser = rosetta.example.PingResponse.parser(),
    )

}