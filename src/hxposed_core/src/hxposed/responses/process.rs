use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::process::GetProcessFieldResponse::{NtPath, Protection, Unknown};
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessResponse {
    pub addr: u64,
}

#[derive(Clone, Default, Debug)]
pub enum GetProcessFieldResponse {
    #[default]
    Unknown,
    NtPath(u16),
    Protection(u32),
}

impl VmcallResponse for GetProcessFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from(raw.result));
        }

        Ok(match raw.arg1 {
            0 => NtPath(raw.arg2 as _),
            1 => Protection(raw.arg2 as _),
            _ => Unknown,
        })
    }

    fn into_raw(self) -> HypervisorResponse {
        let (arg1, arg2, arg3) = match self {
            NtPath(x) => (x as _, u64::MAX, u64::MAX),
            Protection(x) => (x as _, u64::MAX, u64::MAX),
            _ => (u64::MAX, u64::MAX, u64::MAX),
        };

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetProcessField),
            arg1,
            arg2,
            arg3,
        }
    }
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
