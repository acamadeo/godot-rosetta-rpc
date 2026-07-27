use protobuf_gen::rosetta::example::{CurrentTimeRequest, CurrentTimeResponse};

use crate::rosetta::example::Clock;

pub struct ClockImpl;

impl Clock for ClockImpl {
    fn current_time(&self, _request: CurrentTimeRequest) -> CurrentTimeResponse {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock is before the Unix epoch")
            .as_millis() as u64;
        CurrentTimeResponse { millis }
    }
}
