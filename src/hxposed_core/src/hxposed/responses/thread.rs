use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Debug, Default)]
pub struct SuspendThreadResponse {
    pub previous_count: u32,
}

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

impl VmcallResponse for GetThreadFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        GetThreadFieldResponse::from_raw_enum(raw.arg1, raw.arg2)
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.into_raw_enum();
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetThreadField),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}

impl VmcallResponse for SuspendThreadResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            previous_count: raw.arg1 as _,
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
