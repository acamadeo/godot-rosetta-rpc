package rosettarpc

/**
 * Thrown by [RpcClient.call] when a call fails — either at the transport/framework level (unknown
 * service/method, decode failure) or because the service implementation itself failed. Callers
 * distinguish these with [code].
 */
class RpcException(val code: RpcErrorCode, message: String) : RuntimeException(message)
