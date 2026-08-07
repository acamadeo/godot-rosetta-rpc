using System.Text;

namespace RosettaRpc;

/// <summary>
/// Wire-level envelope wrapping every response that crosses the RpcClient
/// dispatch boundary, so that errors can be propagated to the client.
///
/// Every language runtime implements the same envelope protocol:
/// 
/// <code>
/// envelope := status_byte ++ payload
/// status_byte == 0x00 (Ok)  -> payload = the encoded Resp protobuf, verbatim
/// status_byte == 0x01 (Err) -> payload = code:i32 (4 bytes LE)
///                                        ++ message_len:u32 (4 bytes LE)
///                                        ++ message (UTF-8 bytes)
/// </code>
/// </summary>
internal static class Envelope
{
    private const byte StatusOk = 0x00;
    private const byte StatusErr = 0x01;

    public static byte[] EncodeOk(byte[] payload)
    {
        var bytes = new byte[1 + payload.Length];
        bytes[0] = StatusOk;
        Array.Copy(payload, 0, bytes, 1, payload.Length);
        return bytes;
    }

    public static byte[] EncodeError(RpcErrorCode code, string message)
    {
        var messageBytes = Encoding.UTF8.GetBytes(message);
        var bytes = new byte[1 + 4 + 4 + messageBytes.Length];
        bytes[0] = StatusErr;
        WriteInt32LE((int)code, bytes, 1);
        WriteUInt32LE((uint)messageBytes.Length, bytes, 5);
        Array.Copy(messageBytes, 0, bytes, 9, messageBytes.Length);
        return bytes;
    }

    /// <summary>
    /// Decodes an envelope, returning the inner payload on success, or
    /// throwing <see cref="RpcException"/> on the error case (or on a
    /// malformed envelope, as <see cref="RpcErrorCode.Decode"/>).
    /// </summary>
    public static byte[] Decode(byte[] bytes)
    {
        if (bytes.Length < 1)
        {
            throw new RpcException(RpcErrorCode.Decode, "empty response envelope");
        }

        var status = bytes[0];
        var rest = bytes.AsSpan(1);
        return status switch
        {
            StatusOk => rest.ToArray(),
            StatusErr => HandleError(rest),
            _ => throw new RpcException(RpcErrorCode.Decode, $"unknown envelope status byte: {status}"),
        };
    }

    private static byte[] HandleError(ReadOnlySpan<byte> payload)
    {
        if (payload.Length < 8)
        {
            throw new RpcException(RpcErrorCode.Decode, "truncated error envelope");
        }
        var code = (RpcErrorCode)ReadInt32LE(payload);
        var messageLength = ReadUInt32LE(payload[4..]);
        var messageBytes = payload[8..];
        if (messageBytes.Length != messageLength)
        {
            throw new RpcException(RpcErrorCode.Decode, "truncated error envelope message");
        }
        var message = Encoding.UTF8.GetString(messageBytes);
        throw new RpcException(code, message);
    }

    private static void WriteInt32LE(int value, byte[] destination, int offset) =>
        WriteUInt32LE(unchecked((uint)value), destination, offset);

    private static void WriteUInt32LE(uint value, byte[] destination, int offset)
    {
        destination[offset] = (byte)value;
        destination[offset + 1] = (byte)(value >> 8);
        destination[offset + 2] = (byte)(value >> 16);
        destination[offset + 3] = (byte)(value >> 24);
    }

    private static int ReadInt32LE(ReadOnlySpan<byte> source) => unchecked((int)ReadUInt32LE(source));

    private static uint ReadUInt32LE(ReadOnlySpan<byte> source) =>
        (uint)source[0] | ((uint)source[1] << 8) | ((uint)source[2] << 16) | ((uint)source[3] << 24);
}
