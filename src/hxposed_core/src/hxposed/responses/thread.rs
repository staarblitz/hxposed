use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Debug, Default)]
pub struct SuspendThreadResponse {
    pub previous_count: u32,
}

impl VmcallResponse for SuspendThreadResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            Err(HypervisorError::from_response(raw))
        } else {
            Ok(Self {
                previous_count: raw.arg1 as _,
            })
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::SuspendResumeThread),
            arg1: self.previous_count as _,
            ..Default::default()
        }
    }
}
