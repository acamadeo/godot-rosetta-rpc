package rosettarpc

/**
 * Stable, wire-level discriminant for an [RpcException]. Must stay in sync across all language
 * runtimes.
 */
enum class RpcErrorCode(val wireValue: Int) {
  UNKNOWN_SERVICE(0),
  UNKNOWN_METHOD(1),
  DECODE(2),

  /**
   * The service implementation itself failed: either it deliberately threw, or an otherwise-
   * uncaught exception was caught on its behalf by `GodotSupport.dispatchBytes`.
   */
  APPLICATION(3);

  companion object {
    fun fromWireValue(value: Int): RpcErrorCode =
        entries.find { it.wireValue == value } ?: APPLICATION
  }
}
