use crate::hxposed::call::{HxResult};
use crate::hxposed::error::{NotAllowedReason, NotFoundReason};
use alloc::string::String;
use alloc::vec::Vec;
use crate::error::HxError;
use crate::hxposed::ObjectType;

pub mod empty;
pub mod memory;
pub mod notify;
pub mod process;
pub mod security;
pub mod status;
pub mod thread;
pub mod io;
pub mod handle;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct HxResponse {
    pub result: HxResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

impl HxResponse {
    pub fn not_allowed(reason: NotAllowedReason) -> Self {
        Self {
            result: HxResult::from_error(HxError::NotAllowed(reason)),
            arg1: reason.into_bits() as _,
            ..Default::default()
        }
    }

    pub fn invalid_params(param: u32) -> Self {
        Self {
            result: HxResult::from_error(HxError::InvalidParameters(param)),
            ..Default::default()
        }
    }

    pub fn nt_error(reason: u32) -> Self {
        Self {
            result: HxResult::from_error(HxError::NtError(reason)),
            arg1: reason as _,
            ..Default::default()
        }
    }

    pub fn not_found_what(what: NotFoundReason) -> Self {
        Self {
            result: HxResult::from_error(HxError::NotFound(what)),
            ..Default::default()
        }
    }
}

pub trait SyscallResponse: Sized + Send + Sync + Unpin {
    fn from_raw(raw: HxResponse) -> Self;
    fn into_raw(self) -> HxResponse;
}

#[derive(Clone, Debug)]
pub struct OpenObjectResponse {
    pub object: ObjectType,
}

impl SyscallResponse for OpenObjectResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            object: ObjectType::from_raw(raw.arg1, raw.arg2),
        }
    }

    fn into_raw(self) -> HxResponse {
        let object = ObjectType::into_raw(self.object);

        HxResponse {
            result: HxResult::ok(),
            arg1: object.0,
            arg2: object.1,
            ..Default::default()
        }
    }
}