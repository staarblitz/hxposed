use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Debug, Clone)]
pub struct MsrIoResponse {
    pub value: u64,
}

impl VmcallResponse for MsrIoResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self { value: raw.arg1 }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::MsrIo),
            arg1: self.value,
            ..Default::default()
        }
    }
}
