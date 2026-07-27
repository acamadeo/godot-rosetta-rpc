// GENERATED CODE — do not edit. Aggregates every service compiled in this invocation.
package rosetta.example

import rosettarpc.RpcClient

class GeneratedServiceFactory(private val rpc: RpcClient) {

    fun clock(): ClockClient = ClockClient(rpc)

    fun gameService(): GameServiceClient = GameServiceClient(rpc)

}