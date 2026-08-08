use godot_rosetta_rpc::ServiceErr;
use protobuf_gen::rosetta::example::{FailRequest, FailResponse};

use crate::rosetta::example::BrokenRust;

/// Deliberately fails every call. For testing error propagation.
pub struct BrokenRustImpl;

impl BrokenRust for BrokenRustImpl {
    fn fail(&self, _request: FailRequest) -> Result<FailResponse, ServiceErr> {
        ServiceErr::msg("BrokenRust: deliberate failure")
    }
}
