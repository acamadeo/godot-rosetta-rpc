// GENERATED CODE — do not edit. Source: rosetta.example.Clock
package rosetta.example

import rosettarpc.RpcClient

class ClockClient(private val rpc: RpcClient) : Clock {

    override fun currentTime(request: rosetta.example.CurrentTimeRequest): rosetta.example.CurrentTimeResponse =
        rpc.call(ClockDescriptors.CURRENT_TIME, request)

}