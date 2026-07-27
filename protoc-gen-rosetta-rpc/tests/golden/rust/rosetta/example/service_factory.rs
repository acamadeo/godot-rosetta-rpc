// GENERATED CODE — do not edit. Aggregates every service compiled in this invocation.
#[derive(Clone)]
pub struct GeneratedServiceFactory {
    rpc: godot_rosetta_rpc::RpcClient,
}

impl GeneratedServiceFactory {
    pub fn new(rpc: godot_rosetta_rpc::RpcClient) -> Self {
        Self { rpc }
    }

    pub fn clock(&self) -> ClockClient {
        ClockClient::new(self.rpc.clone())
    }

    pub fn game_service(&self) -> GameServiceClient {
        GameServiceClient::new(self.rpc.clone())
    }

}