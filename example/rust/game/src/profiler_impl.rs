use crate::rosetta::example::{Clock, GeneratedServiceFactory};
use protobuf_gen::rosetta::example::{CurrentTimeRequest, ProfileRequest, ProfileResponse};

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
    fn profile(&self, _request: ProfileRequest) -> ProfileResponse {
        let time0 = self
            .factory
            .clock()
            .current_time(CurrentTimeRequest {})
            .millis;
        let time1 = self
            .factory
            .clock()
            .current_time(CurrentTimeRequest {})
            .millis;
        ProfileResponse {
            message: format!("Consecutive clock cycles ran in {} ms", time1 - time0),
        }
    }
}
