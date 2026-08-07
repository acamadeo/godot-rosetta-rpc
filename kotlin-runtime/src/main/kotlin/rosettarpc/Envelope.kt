package rosettarpc

import java.nio.charset.StandardCharsets

/**
 * Wire-level envelope wrapping every response that crosses the [RpcClient]/[ServiceRegistry]
 * dispatch boundary, so a call failure can travel back to the caller as data instead of relying on
 * an exception surviving Godot's `Variant` call boundary between language runtimes.
 *
 * Deliberately hand-rolled rather than a protobuf message: this framing is purely internal to this
 * library, never part of a project's `.proto` schema. The Rust and C# runtimes implement the same
 * framing — keep them in sync if this changes.
 *
 * ```
 * envelope := status_byte ++ payload
 * status_byte == 0x00 (Ok)  -> payload = the encoded Resp protobuf, verbatim
 * status_byte == 0x01 (Err) -> payload = code:i32 (4 bytes LE)
 *                                        ++ message_len:u32 (4 bytes LE)
 *                                        ++ message (UTF-8 bytes)
 * ```
 */
internal object Envelope {
  private const val STATUS_OK: Byte = 0x00
  private const val STATUS_ERR: Byte = 0x01

  fun encodeOk(payload: ByteArray): ByteArray = byteArrayOf(STATUS_OK) + payload

  fun encodeError(code: RpcErrorCode, message: String): ByteArray {
    val messageBytes = message.toByteArray(StandardCharsets.UTF_8)
    val bytes = ByteArray(1 + 4 + 4 + messageBytes.size)
    bytes[0] = STATUS_ERR
    writeInt32LE(code.wireValue, bytes, 1)
    writeInt32LE(messageBytes.size, bytes, 5)
    messageBytes.copyInto(bytes, destinationOffset = 9)
    return bytes
  }

  /**
   * Decodes an envelope, returning the inner payload on success, or throwing [RpcException] on the
   * error case (or on a malformed envelope, as [RpcErrorCode.DECODE]).
   */
  fun decode(bytes: ByteArray): ByteArray {
    if (bytes.isEmpty()) {
      throw RpcException(RpcErrorCode.DECODE, "empty response envelope")
    }
    val status = bytes[0]
    val rest = bytes.copyOfRange(1, bytes.size)
    return when (status) {
      STATUS_OK -> rest
      STATUS_ERR -> handleError(rest)
      else -> throw RpcException(RpcErrorCode.DECODE, "unknown envelope status byte: $status")
    }
  }

  private fun handleError(payload: ByteArray): ByteArray {
    if (payload.size < 8) {
      throw RpcException(RpcErrorCode.DECODE, "truncated error envelope")
    }
    val code = RpcErrorCode.fromWireValue(readInt32LE(payload, 0))
    val messageLength = readInt32LE(payload, 4)
    val messageBytes = payload.copyOfRange(8, payload.size)
    if (messageBytes.size != messageLength) {
      throw RpcException(RpcErrorCode.DECODE, "truncated error envelope message")
    }
    val message = String(messageBytes, StandardCharsets.UTF_8)
    throw RpcException(code, message)
  }

  private fun writeInt32LE(value: Int, destination: ByteArray, offset: Int) {
    destination[offset] = (value and 0xFF).toByte()
    destination[offset + 1] = ((value shr 8) and 0xFF).toByte()
    destination[offset + 2] = ((value shr 16) and 0xFF).toByte()
    destination[offset + 3] = ((value shr 24) and 0xFF).toByte()
  }

  private fun readInt32LE(source: ByteArray, offset: Int): Int =
      (source[offset].toInt() and 0xFF) or
          ((source[offset + 1].toInt() and 0xFF) shl 8) or
          ((source[offset + 2].toInt() and 0xFF) shl 16) or
          ((source[offset + 3].toInt() and 0xFF) shl 24)
}
