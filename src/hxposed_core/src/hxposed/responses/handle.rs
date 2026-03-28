use crate::hxposed::call::HxResult;
use crate::hxposed::responses::{HxResponse, SyscallResponse};

#[derive(Clone)]
pub struct GetHandleObjectResponse {
    pub object: u64,
    pub granted_access: u32
}

impl SyscallResponse for GetHandleObjectResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            object: raw.arg1,
            granted_access: raw.arg2 as _
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.object,
            arg2: self.granted_access as _,
            ..Default::default()
        }
    }
}