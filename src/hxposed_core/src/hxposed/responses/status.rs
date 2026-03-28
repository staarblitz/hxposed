use crate::hxposed::call::HxResult;
use crate::hxposed::responses::{HxResponse, SyscallResponse};
use crate::hxposed::status::HypervisorStatus;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct StatusResponse {
    pub state: HypervisorStatus,
    pub version: u32,
}

impl SyscallResponse for StatusResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            state: HypervisorStatus::from(raw.arg1 as u32),
            version: raw.arg2 as _,
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.state as _,
            arg2: self.version as _,
            ..Default::default()
        }
    }
}
