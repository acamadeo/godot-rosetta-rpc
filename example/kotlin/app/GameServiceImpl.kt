package example.app

import rosetta.example.GameService
import rosetta.example.GeneratedServiceFactory
import rosetta.example.PingRequest
import rosetta.example.PingResponse
import rosetta.example.currentTimeRequest
import rosetta.example.pingResponse

/**
 * GameService is implemented in Kotlin and calls Clock — implemented in Rust — purely through the
 * generated ClockClient, with no idea Clock is implemented in a different language.
 */
class GameServiceImpl(private val factory: GeneratedServiceFactory) : GameService {
  override fun ping(request: PingRequest): PingResponse {
    val clock = factory.clock()
    val currentTime = clock.currentTime(currentTimeRequest {})
    return pingResponse { message = "pong: ${request.message} @ ${currentTime.millis}ms" }
  }
}
