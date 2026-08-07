package example.app

import rosetta.example.GameService
import rosetta.example.GeneratedServiceFactory
import rosetta.example.PingRequest
import rosetta.example.PingResponse
import rosetta.example.ProbeBrokenServiceRequest
import rosetta.example.ProbeBrokenServiceResult
import rosetta.example.currentTimeRequest
import rosetta.example.failRequest
import rosetta.example.pingResponse
import rosetta.example.probeBrokenServiceResult
import rosettarpc.RpcException

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

  /**
   * Calls the BrokenXXX service named by `request.Target` and reports the RpcException it caught.
   *
   * Note that Kotlin can only call Rust across language boundaries.
   */
  override fun probeBrokenService(request: ProbeBrokenServiceRequest): ProbeBrokenServiceResult {
    return try {
      when (request.target) {
        "BrokenRust" -> factory.brokenRust().fail(failRequest {})
        else -> throw IllegalArgumentException("incompatible probe target: ${request.target}")
      }
      probeBrokenServiceResult { errorMessage = "" }
    } catch (e: RpcException) {
      probeBrokenServiceResult { errorMessage = e.message ?: "" }
    }
  }
}
