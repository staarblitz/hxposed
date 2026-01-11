use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum GetProcessFieldResponse {
    NtPath(u16) = 1,
    Protection(u32) = 2,
    Signers(u16) = 3,
    Mitigation(u64) = 4,
    Token(u64) = 5,
}

impl GetProcessFieldResponse {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            GetProcessFieldResponse::NtPath(ntpath) => (1, ntpath as u64),
            GetProcessFieldResponse::Protection(protection) => (2, protection as u64),
            GetProcessFieldResponse::Signers(signers) => (3, signers as u64),
            GetProcessFieldResponse::Mitigation(mitigation) => (4, mitigation),
            GetProcessFieldResponse::Token(token) => (5, token),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> GetProcessFieldResponse {
        match object {
            1 => GetProcessFieldResponse::NtPath(value as _),
            2 => GetProcessFieldResponse::Protection(value as _),
            3 => GetProcessFieldResponse::Signers(value as _),
            4 => GetProcessFieldResponse::Mitigation(value as _),
            5 => GetProcessFieldResponse::Token(value as _),
            _ => unreachable!("Invalid object id: {}", object),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetProcessThreadsResponse {
    pub number_of_threads: u32,
}

impl VmcallResponse for GetProcessThreadsResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            number_of_threads: raw.arg1 as _,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetProcessThreads),
            arg1: self.number_of_threads as _,

            ..Default::default()
        }
    }
}

impl VmcallResponse for GetProcessFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        GetProcessFieldResponse::from_raw_enum(raw.arg1 as _, raw.arg2 as _)
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.into_raw_enum();
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetProcessField),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}
