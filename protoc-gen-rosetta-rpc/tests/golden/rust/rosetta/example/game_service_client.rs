// GENERATED CODE — do not edit. Source: rosetta.example.GameService
#[derive(Clone)]
pub struct GameServiceClient {
    rpc: godot_rosetta_rpc::RpcClient,
}

impl GameServiceClient {
    pub fn new(rpc: godot_rosetta_rpc::RpcClient) -> Self {
        Self { rpc }
    }
}

impl GameService for GameServiceClient {

    fn ping(&self, request: protobuf_gen::rosetta::example::PingRequest) -> protobuf_gen::rosetta::example::PingResponse {
        self.rpc.call(&GameServiceDescriptors::PING, request)
    }

}