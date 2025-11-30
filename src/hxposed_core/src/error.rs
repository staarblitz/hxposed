use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorSource, InternalErrorCode};
use static_assertions::assert_eq_size;
use crate::hxposed::responses::HypervisorResponse;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub struct HypervisorError {
    pub error_source: ErrorSource,
    pub error_code: u16,
    pub error_reason: u16
}
assert_eq_size!(HypervisorError, u64);

impl HypervisorError {
    pub fn ok() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::Ok as _,
            error_reason: InternalErrorCode::Ok as _
        }
    }

    pub fn not_found() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::NotFound as _,
            error_reason: InternalErrorCode::NotFound as _
        }
    }

    pub fn is_err(&self) -> bool {
        !(self.error_code == InternalErrorCode::Ok as _ && self.error_source == ErrorSource::Hx)
    }

    pub fn from_response(response: HypervisorResponse) -> HypervisorError {
        Self {
            error_source: response.result.error_source(),
            error_code: response.result.error_code() as _,
            error_reason: response.arg1 as _
        }
    }
}