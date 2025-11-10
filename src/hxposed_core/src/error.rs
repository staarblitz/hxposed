use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::{ErrorCode, ErrorSource};
use bitfield_struct::bitfield;
use static_assertions::assert_eq_size;

#[repr(C)]
pub struct HypervisorError {
    pub error_source: ErrorSource,
    pub error_code: ErrorCode,
}
assert_eq_size!(HypervisorError, u64);

impl HypervisorError {
    pub fn ok() -> HypervisorError {
        Self{
            error_source: ErrorSource::Hx,
            error_code: ErrorCode::Ok,
        }
    }

    pub fn is_err(&self) -> bool {
        !(self.error_code == ErrorCode::Ok && self.error_source == ErrorSource::Hx)
    }
}

impl From<HypervisorResult> for HypervisorError {
    fn from(value: HypervisorResult) -> Self {
        Self{
            error_source: value.error_source(),
            error_code: value.error_code(),
        }
    }
}
