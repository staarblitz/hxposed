use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::status::HypervisorStatus;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessResponse {
    pub addr: u64,
}

impl VmcallResponse for OpenProcessResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from(raw.result));
        }
        Ok(Self { addr: raw.arg1 })
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::OpenProcess),
            arg1: self.addr,
            ..Default::default()
        }
    }
}
