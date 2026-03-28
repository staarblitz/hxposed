use crate::hxposed::call::HxCall;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::status::StatusResponse;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct StatusRequest;

impl SyscallRequest for StatusRequest {
    type Response = StatusResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::get_status(),
            ..Default::default()
        }
    }

    fn from_raw(_request: &HxRequest) -> Self {
        Self {}
    }
}
