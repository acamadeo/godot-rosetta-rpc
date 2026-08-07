// GENERATED CODE — do not edit. Source: rosetta.example.Clock
pub struct ClockAdapter {
    pub implementation: Box<dyn Clock>,
}

impl godot_rosetta_rpc::ErasedAdapter for ClockAdapter {
    fn invoke(&self, method_id: &str, request_bytes: &[u8]) -> Result<Vec<u8>, godot_rosetta_rpc::RpcError> {
        match method_id {

            "CurrentTime" => {
                let request = <protobuf_gen::rosetta::example::CurrentTimeRequest as prost::Message>::decode(request_bytes)
                    .map_err(|_| godot_rosetta_rpc::RpcError::Decode)?;
                let response = self.implementation.current_time(request)?;
                Ok(prost::Message::encode_to_vec(&response))
            }

            _ => Err(godot_rosetta_rpc::RpcError::UnknownMethod),
        }
    }
}