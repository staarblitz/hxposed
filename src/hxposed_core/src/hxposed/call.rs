use crate::hxposed::error::{ErrorCode, ErrorSource};
use crate::hxposed::func::ServiceFunction;
use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct HypervisorCall {
    #[bits(16)]
    pub func: ServiceFunction,
    pub is_fast: bool,
    pub ignore_result: bool,
    pub buffer_by_user: bool,
    pub yield_execution: bool,
    pub is_async: bool,

    #[bits(11)]
    pub async_cookie: u16,
}

impl HypervisorCall {
    pub(crate) fn get_status() -> Self {
        // For this call, other fields are ignored.
        Self::new().with_func(ServiceFunction::GetState)
    }

    pub(crate) fn auth() -> Self {
        Self::new().with_func(ServiceFunction::Authorize)
    }

    pub(crate) fn open_process() -> Self {
        Self::new().with_func(ServiceFunction::OpenProcess)
    }
}

#[bitfield(u32)]
pub struct HypervisorResult {
    #[bits(16)]
    pub func: ServiceFunction,
    #[bits(2)]
    pub error_source: ErrorSource,
    #[bits(3)]
    pub error_code: ErrorCode,
    #[bits(11)]
    pub cookie: u16,
}

impl HypervisorResult {
    pub fn ok(func: ServiceFunction) -> Self {
        Self::error(ErrorSource::Hx, ErrorCode::Ok).with_func(func)
    }

    pub fn error(error_source: ErrorSource, error_code: ErrorCode) -> Self {
        Self::error_with_cookie(error_source, error_code, 0)
    }

    pub fn error_with_cookie(
        error_source: ErrorSource,
        error_code: ErrorCode,
        cookie: u16,
    ) -> Self {
        HypervisorResult::default()
            .with_error_source(error_source)
            .with_error_code(error_code)
            .with_cookie(cookie)
    }
}
