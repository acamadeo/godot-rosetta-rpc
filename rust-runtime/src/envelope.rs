//! Wire-level envelope wrapping every response that crosses the
//! `RpcClient`/`ServiceRegistry` dispatch boundary, so a call failure can
//! travel back to the caller as data instead of relying on a panic/exception
//! surviving Godot's `Variant` call boundary between language runtimes.
//!
//! Deliberately hand-rolled rather than a protobuf message: this framing is
//! purely internal to this library (never part of a project's `.proto`
//! schema), so it doesn't need protobuf's evolvability guarantees, and
//! keeping it out of protobuf avoids a bootstrapping dependency on
//! project-generated code. The C# and Kotlin runtimes implement the same
//! framing; keep them in sync if this changes.
//!
//! ```text
//! envelope := status_byte ++ payload
//! status_byte == 0x00 (Ok)  -> payload = the encoded Resp protobuf, verbatim
//! status_byte == 0x01 (Err) -> payload = code:i32 (4 bytes LE)
//!                                     ++ message_len:u32 (4 bytes LE)
//!                                     ++ message (UTF-8 bytes)
//! ```

use crate::error::RpcError;

const STATUS_OK: u8 = 0x00;
const STATUS_ERR: u8 = 0x01;

pub fn encode_ok(payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(1 + payload.len());
    bytes.push(STATUS_OK);
    bytes.extend_from_slice(payload);
    bytes
}

pub fn encode_err(error: &RpcError) -> Vec<u8> {
    let message = error.message();
    let message_bytes = message.as_bytes();
    let mut bytes = Vec::with_capacity(1 + 4 + 4 + message_bytes.len());
    bytes.push(STATUS_ERR);
    bytes.extend_from_slice(&error.code().to_le_bytes());
    bytes.extend_from_slice(&(message_bytes.len() as u32).to_le_bytes());
    bytes.extend_from_slice(message_bytes);
    bytes
}

/// Decodes an envelope, returning the inner payload on success. A malformed
/// envelope (too short, bad UTF-8) is itself reported as `RpcError::Decode`.
pub fn decode(bytes: &[u8]) -> Result<Vec<u8>, RpcError> {
    let (&status, rest) = bytes.split_first().ok_or(RpcError::Decode)?;
    match status {
        STATUS_OK => Ok(rest.to_vec()),
        STATUS_ERR => {
            if rest.len() < 8 {
                return Err(RpcError::Decode);
            }
            let (code_bytes, rest) = rest.split_at(4);
            let code = i32::from_le_bytes(code_bytes.try_into().unwrap());
            let (len_bytes, rest) = rest.split_at(4);
            let message_len = u32::from_le_bytes(len_bytes.try_into().unwrap()) as usize;
            if rest.len() != message_len {
                return Err(RpcError::Decode);
            }
            let message = String::from_utf8(rest.to_vec()).map_err(|_| RpcError::Decode)?;
            Err(RpcError::from_code(code, message))
        }
        _ => Err(RpcError::Decode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_ok_payload() {
        let bytes = encode_ok(b"hello");
        assert_eq!(decode(&bytes), Ok(b"hello".to_vec()));
    }

    #[test]
    fn round_trips_error() {
        let error = RpcError::Application("boom".to_string());
        let bytes = encode_err(&error);
        assert_eq!(decode(&bytes), Err(error));
    }

    #[test]
    fn round_trips_every_known_variant() {
        for error in [
            RpcError::UnknownService,
            RpcError::UnknownMethod,
            RpcError::Decode,
            RpcError::Application("custom message".to_string()),
        ] {
            let bytes = encode_err(&error);
            assert_eq!(decode(&bytes), Err(error));
        }
    }

    #[test]
    fn empty_bytes_decode_as_decode_error() {
        assert_eq!(decode(&[]), Err(RpcError::Decode));
    }

    #[test]
    fn truncated_error_envelope_decodes_as_decode_error() {
        assert_eq!(decode(&[STATUS_ERR, 0, 0]), Err(RpcError::Decode));
    }
}
