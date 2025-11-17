use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorCode, ErrorSource, NotAllowedReason};
use crate::plugins::plugin_perms::PluginPermissions;

pub mod auth;
pub mod process;
pub mod status;

#[derive(Copy, Clone, Default, Debug)]
pub struct HypervisorResponse {
    pub result: HypervisorResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

impl HypervisorResponse {
    pub fn not_allowed(reason: NotAllowedReason, permissions: PluginPermissions) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, ErrorCode::NotAllowed),
            arg1: reason.into_bits() as _,
            arg2: permissions.bits(),
            arg3: 0,
        }
    }

    pub fn nt_error(reason: u32) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, ErrorCode::Unknown),
            arg1: reason as _,
            arg2: 0,
            arg3: 0
        }
    }
}

pub trait VmcallResponse: Sized {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError>;
    fn into_raw(self) -> HypervisorResponse;
}
