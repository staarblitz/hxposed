use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::response::{HypervisorResponse, VmcallResponse};
use crate::hxposed::status::HypervisorStatus;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct StatusResponse {
    pub state: HypervisorStatus,
    pub version: u32,
}

impl VmcallResponse for StatusResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            state: HypervisorStatus::from(raw.arg1 as u32),
            version: raw.arg2 as _,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetState),
            arg1: self.state as _,
            arg2: self.version as _,
            arg3: 0,
        }
    }
}
