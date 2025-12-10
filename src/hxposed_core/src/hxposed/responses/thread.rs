use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::responses::process::GetProcessFieldResponse;

#[derive(Clone, Debug, Default)]
pub struct SuspendThreadResponse {
    pub previous_count: u32,
}

#[derive(Clone, Default, Debug)]
#[repr(u16)]
pub enum GetThreadFieldResponse {
    #[default]
    Unknown = 0,
    ActiveImpersonationInfo(bool) = 1,
    AdjustedClientToken(u64) = 2,
}

impl VmcallResponse for GetThreadFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from_response(raw));
        }

        Ok(match raw.arg1 {
            1 => Self::ActiveImpersonationInfo(raw.arg2 == 1),
            _ => unreachable!("Developer forgot to implement this one."),
        })
    }

    fn into_raw(self) -> HypervisorResponse {
        let (arg1, arg2, arg3) = match self {
            Self::ActiveImpersonationInfo(arg) => (1, arg as u64, 0),
            Self::AdjustedClientToken(arg) => (2, arg, 0),
            Self::Unknown => unreachable!(), // didn't use _ => on purpose, so I never forget implementing new ones
        };

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetThreadField),
            arg1,
            arg2,
            arg3,
        }
    }
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
