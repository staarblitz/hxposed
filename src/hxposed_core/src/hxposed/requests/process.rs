use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::process::OpenProcessResponse;
use crate::intern::instructions::{vmcall, vmcall_typed};

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessRequest {
    pub process_id: u32,
}

impl VmcallRequest for OpenProcessRequest {
    type Response = OpenProcessResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::open_process(),
            arg1: self.process_id as _,
            arg2: 0,
            arg3: 0
        }
    }
}