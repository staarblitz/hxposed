use crate::hxposed::call::HxResult;
use crate::hxposed::responses::{HxResponse, SyscallResponse};

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum GetThreadFieldResponse {
    ActiveImpersonationInfo(bool) = 1,
    AdjustedClientToken(u64) = 2,
}

impl GetThreadFieldResponse {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            GetThreadFieldResponse::ActiveImpersonationInfo(info) => (0, info as u64),
            GetThreadFieldResponse::AdjustedClientToken(token) => (1, token),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            0 => GetThreadFieldResponse::ActiveImpersonationInfo(value == 1),
            1 => GetThreadFieldResponse::AdjustedClientToken(value),
            _ => panic!("Invalid object id: {}", object),
        }
    }
}

impl SyscallResponse for GetThreadFieldResponse {
    fn from_raw(raw: HxResponse) -> Self {
        GetThreadFieldResponse::from_raw_enum(raw.arg1, raw.arg2)
    }

    fn into_raw(self) -> HxResponse {
        let args = self.into_raw_enum();
        HxResponse {
            result: HxResult::ok(),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}