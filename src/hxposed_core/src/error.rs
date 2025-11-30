use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorSource, InternalErrorCode};
use static_assertions::assert_eq_size;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub struct HypervisorError {
    pub error_source: ErrorSource,
    pub error_code: u32,
}
assert_eq_size!(HypervisorError, u64);

impl HypervisorError {
    pub fn ok() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::Ok as _,
        }
    }

    pub fn not_found() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::NotFound as _,
        }
    }

    pub fn is_err(&self) -> bool {
        !(self.error_code == InternalErrorCode::Ok as _ && self.error_source == ErrorSource::Hx)
    }
}

impl From<HypervisorResult> for HypervisorError {
    fn from(value: HypervisorResult) -> Self {
        Self {
            error_source: value.error_source(),
            error_code: value.error_code() as _,
        }
    }
}
