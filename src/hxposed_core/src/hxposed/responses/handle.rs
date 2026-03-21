use crate::hxposed::call::HypervisorResult;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone)]
pub struct GetHandleObjectResponse {
    pub object: u64,
    pub granted_access: u32
}

impl VmcallResponse for GetHandleObjectResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            object: raw.arg1,
            granted_access: raw.arg2 as _
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: self.object,
            arg2: self.granted_access as _,
            ..Default::default()
        }
    }
}