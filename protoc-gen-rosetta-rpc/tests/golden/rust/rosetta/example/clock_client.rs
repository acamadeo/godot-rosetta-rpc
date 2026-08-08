// GENERATED CODE — do not edit. Source: rosetta.example.Clock
#[derive(Clone)]
pub struct ClockClient {
    rpc: godot_rosetta_rpc::RpcClient,
}

impl ClockClient {
    pub fn new(rpc: godot_rosetta_rpc::RpcClient) -> Self {
        Self { rpc }
    }

    pub fn current_time(&self, request: protobuf_gen::rosetta::example::CurrentTimeRequest) -> Result<protobuf_gen::rosetta::example::CurrentTimeResponse, godot_rosetta_rpc::RpcError> {
        self.rpc.call(&ClockDescriptors::CURRENT_TIME, request)
    }

}