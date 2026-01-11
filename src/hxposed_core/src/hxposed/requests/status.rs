use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::status::StatusResponse;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct StatusRequest;

impl VmcallRequest for StatusRequest {
    type Response = StatusResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::get_status(),
            ..Default::default()
        }
    }

    fn from_raw(_request: &HypervisorRequest) -> Self {
        Self {}
    }
}
