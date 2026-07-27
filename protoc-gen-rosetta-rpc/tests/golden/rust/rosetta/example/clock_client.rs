// GENERATED CODE — do not edit. Source: rosetta.example.Clock
#[derive(Clone)]
pub struct ClockClient {
    rpc: godot_rosetta_rpc::RpcClient,
}

impl ClockClient {
    pub fn new(rpc: godot_rosetta_rpc::RpcClient) -> Self {
        Self { rpc }
    }
}

impl Clock for ClockClient {

    fn current_time(&self, request: protobuf_gen::rosetta::example::CurrentTimeRequest) -> protobuf_gen::rosetta::example::CurrentTimeResponse {
        self.rpc.call(&ClockDescriptors::CURRENT_TIME, request)
    }

}