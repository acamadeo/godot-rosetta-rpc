// GENERATED CODE — do not edit. Source: rosetta.example.GameService
pub struct GameServiceAdapter {
    pub implementation: Box<dyn GameService>,
}

impl godot_rosetta_rpc::ErasedAdapter for GameServiceAdapter {
    fn invoke(&self, method_id: &str, request_bytes: &[u8]) -> Result<Vec<u8>, godot_rosetta_rpc::RpcError> {
        match method_id {

            "Ping" => {
                let request = <protobuf_gen::rosetta::example::PingRequest as prost::Message>::decode(request_bytes)
                    .map_err(|_| godot_rosetta_rpc::RpcError::Decode)?;
                let response = self.implementation.ping(request)?;
                Ok(prost::Message::encode_to_vec(&response))
            }

            _ => Err(godot_rosetta_rpc::RpcError::UnknownMethod),
        }
    }
}