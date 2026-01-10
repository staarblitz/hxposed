use crate::error::HypervisorError;
use crate::hxposed::call::{HypervisorCall, HypervisorResult, ServiceParameter};
use crate::hxposed::error::{ErrorSource, InternalErrorCode, NotAllowedReason, NotFoundReason};
use crate::plugins::plugin_perms::PluginPermissions;

pub mod auth;
pub mod empty;
pub mod memory;
pub mod process;
pub mod security;
pub mod status;
pub mod thread;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct HypervisorResponse {
    pub result: HypervisorResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

impl HypervisorResponse {
    pub fn not_allowed(reason: NotAllowedReason) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotAllowed),
            arg1: reason.into_bits() as _,
            ..Default::default()
        }
    }

    pub fn invalid_params(param: ServiceParameter) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::InvalidParams),
            arg1: param.into_bits() as _,
            ..Default::default()
        }
    }

    pub fn not_allowed_perms(permissions: PluginPermissions) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotAllowed),
            arg1: NotAllowedReason::MissingPermissions as _,
            arg2: permissions.bits(),
            arg3: 0,
        }
    }

    pub fn nt_error(reason: u32) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Nt, InternalErrorCode::Unknown),
            arg1: reason as _,
            ..Default::default()
        }
    }

    pub fn not_found() -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotFound),
            ..Default::default()
        }
    }

    pub fn not_found_what(what: NotFoundReason) -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::NotFound),
            arg1: what.into_bits() as _,
            ..Default::default()
        }
    }
}

pub trait VmcallResponse: Sized + Send + Sync + Unpin {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError>;
    fn into_raw(self) -> HypervisorResponse;
}
