// GENERATED CODE — do not edit. Source: rosetta.example.GameService
package rosetta.example

import rosettarpc.RpcClient

class GameServiceClient(private val rpc: RpcClient) : GameService {

    override fun ping(request: rosetta.example.PingRequest): rosetta.example.PingResponse =
        rpc.call(GameServiceDescriptors.PING, request)

}