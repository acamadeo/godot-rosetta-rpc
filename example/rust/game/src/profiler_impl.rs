use crate::rosetta::example::GeneratedServiceFactory;
use godot_rosetta_rpc::ServiceErr;
use protobuf_gen::rosetta::example::{
    CurrentTimeRequest, FailRequest, ProbeBrokenServiceRequest, ProbeBrokenServiceResult,
    ProfileRequest, ProfileResponse,
};

use crate::rosetta::example::Profiler;

pub struct ProfilerImpl {
    factory: GeneratedServiceFactory,
}

impl ProfilerImpl {
    pub fn new(factory: GeneratedServiceFactory) -> Self {
        Self { factory }
    }
}

impl Profiler for ProfilerImpl {
    fn profile(&self, _request: ProfileRequest) -> Result<ProfileResponse, ServiceErr> {
        let time0 = self
            .factory
            .clock()
            .current_time(CurrentTimeRequest {})?
            .millis;
        let time1 = self
            .factory
            .clock()
            .current_time(CurrentTimeRequest {})?
            .millis;
        Ok(ProfileResponse {
            message: format!("Consecutive clock cycles ran in {} ms", time1 - time0),
        })
    }

    /// Calls the BrokenXXX service named by `request.target` and reports the
    /// error it caught.
    fn probe_broken_service(
        &self,
        request: ProbeBrokenServiceRequest,
    ) -> Result<ProbeBrokenServiceResult, ServiceErr> {
        let result = match request.target.as_str() {
            "BrokenKotlin" => self.factory.broken_kotlin().fail(FailRequest {}),
            "BrokenCSharp" => self.factory.broken_c_sharp().fail(FailRequest {}),
            other => return ServiceErr::msg(format!("incompatible probe target: {other}")),
        };
        // Return the error message from the call, or an empty string if we received
        // an unexpected success.
        Ok(ProbeBrokenServiceResult {
            error_message: result.err().map(|e| e.to_string()).unwrap_or_default(),
        })
    }
}
