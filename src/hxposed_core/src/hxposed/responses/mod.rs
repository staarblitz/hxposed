use crate::hxposed::call::{HypervisorResult, ServiceParameter};
use crate::hxposed::error::{ErrorSource, InternalErrorCode, NotAllowedReason, NotFoundReason};
use alloc::string::String;
use alloc::vec::Vec;

pub mod empty;
pub mod memory;
pub mod notify;
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

    pub fn invalid_param() -> Self {
        Self {
            result: HypervisorResult::error(ErrorSource::Hx, InternalErrorCode::InvalidParams),
            ..Default::default()
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
    fn from_raw(raw: HypervisorResponse) -> Self;
    fn into_raw(self) -> HypervisorResponse;
}

// messy. ideas?

pub const RESPONSE_BASE: u64 = 0x20090000;
pub const CALLBACK_BASE: u64 = 0x40180000;

pub unsafe fn read_response_length(offset: u64) -> u32 {
    unsafe { *((RESPONSE_BASE + offset) as *const u32) }
}

pub unsafe fn read_response_data<T>(offset: u64) -> Vec<T>
where
    T: Clone,
{
    unsafe {
        let count = read_response_length(offset);
        // from_raw_parts does not copy. so if we make enough calls our string will be corrupted.
        // so we copy!
        core::slice::from_raw_parts((RESPONSE_BASE + (offset) + 4) as *const T, count as _)
            .iter()
            .cloned()
            .collect::<Vec<T>>()
    }
}

/// Only use if you know what you are doing
pub unsafe fn read_response_data_no_copy<T>(offset: u64) -> &'static [T]
where
    T: Clone,
{
    unsafe {
        let count = read_response_length(offset);
        // from_raw_parts does not copy. so if we make enough calls our string will be corrupted.
        // so we copy!
        core::slice::from_raw_parts((RESPONSE_BASE + (offset) + 4) as *const T, count as _)
    }
}

pub unsafe fn read_response_type<T>(offset: u64) -> T
where
    T: Clone,
{
    unsafe {
        let type_offset = read_response_length(offset);
        (*((RESPONSE_BASE + (type_offset as u64)) as *const T)).clone()
    }
}

pub fn read_response_as_string(offset: u64) -> String {
    let data = unsafe { read_response_data_no_copy::<u16>(offset) };
    // from_utf16 internally makes a copy. so its "safe"
    String::from_utf16(data).unwrap()
}
