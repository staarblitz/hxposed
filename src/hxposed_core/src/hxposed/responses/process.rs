use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessResponse {
    pub addr: u64,
}

#[derive(Clone, Default, Debug)]
#[repr(u16)]
pub enum GetProcessFieldResponse {
    #[default]
    Unknown = 0,
    NtPath(u16) = 1,
    Protection(u32) = 2,
    Signers(u16) = 3
}

impl VmcallResponse for GetProcessFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from_response(raw));
        }

        Ok(match raw.arg1 {
            1 => Self::NtPath(raw.arg2 as _),
            2 => Self::Protection(raw.arg2 as _),
            3 => Self::Signers(raw.arg2 as _),
            _ => unreachable!("Developer forgot to implement this one.")
        })
    }

    fn into_raw(self) -> HypervisorResponse {
        let (arg1, arg2, arg3) = match self {
            Self::NtPath(x) => (1, x as _, 0),
            Self::Protection(x) => (2, x as _, 0),
            Self::Signers(x) => (3, x as _, 0),
            GetProcessFieldResponse::Unknown => unreachable!(), // didn't use _ => on purpose, so I never forget implementing new ones
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
            return Err(HypervisorError::from_response(raw));
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
