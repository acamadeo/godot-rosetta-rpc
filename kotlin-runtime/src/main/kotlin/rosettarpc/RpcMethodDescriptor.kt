package rosettarpc

import com.google.protobuf.Message
import com.google.protobuf.Parser

/**
 * Identifies one RPC method: a service id, a method id, and the parsers needed to decode a
 * request/response of that method's types.
 */
class RpcMethodDescriptor<Req : Message, Resp : Message>(
    val serviceId: String,
    val methodId: String,
    val requestParser: Parser<Req>,
    val responseParser: Parser<Resp>,
)
