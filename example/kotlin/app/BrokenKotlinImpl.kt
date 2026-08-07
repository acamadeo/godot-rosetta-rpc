package example.app

import rosetta.example.BrokenKotlin
import rosetta.example.FailRequest
import rosetta.example.FailResponse

/** Deliberately fails every call. For testing error propagation. */
class BrokenKotlinImpl : BrokenKotlin {
  override fun fail(request: FailRequest): FailResponse {
    throw IllegalStateException("BrokenKotlin: deliberate failure")
  }
}
