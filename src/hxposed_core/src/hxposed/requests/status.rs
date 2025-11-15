use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::status::StatusResponse;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct StatusRequest;

impl VmcallRequest for StatusRequest {
    type Response = StatusResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::get_status(),
            arg1: 0,
            arg2: 0,
            arg3: 0,
        }
    }

    fn from_raw(call: HypervisorCall, arg1: u64, arg2: u64, arg3: u64) -> Self {
        Self {}
    }
}
