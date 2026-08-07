// GENERATED CODE — do not edit. Source: rosetta.example.GameService
#[derive(Clone)]
pub struct GameServiceClient {
    rpc: godot_rosetta_rpc::RpcClient,
}

impl GameServiceClient {
    pub fn new(rpc: godot_rosetta_rpc::RpcClient) -> Self {
        Self { rpc }
    }

    pub fn ping(&self, request: protobuf_gen::rosetta::example::PingRequest) -> Result<protobuf_gen::rosetta::example::PingResponse, godot_rosetta_rpc::RpcError> {
        self.rpc.call(&GameServiceDescriptors::PING, request)
    }

}