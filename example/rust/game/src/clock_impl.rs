use godot_rosetta_rpc::ServiceErr;
use protobuf_gen::rosetta::example::{CurrentTimeRequest, CurrentTimeResponse};

use crate::rosetta::example::Clock;

pub struct ClockImpl;

impl Clock for ClockImpl {
    fn current_time(
        &self,
        _request: CurrentTimeRequest,
    ) -> Result<CurrentTimeResponse, ServiceErr> {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis() as u64;
        Ok(CurrentTimeResponse { millis })
    }
}
