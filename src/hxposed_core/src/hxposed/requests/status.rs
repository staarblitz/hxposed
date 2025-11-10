use crate::hxposed::call::HypervisorCall;
use crate::hxposed::request::{HypervisorRequest, VmcallRequest};
use crate::hxposed::response::{HypervisorResponse, VmcallResponse};
use crate::hxposed::responses::status::StatusResponse;
use crate::intern::instructions::vmcall;

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

    fn send(self) -> Self::Response {
        Self::Response::from_raw(vmcall(self.into_raw()))
    }
}
