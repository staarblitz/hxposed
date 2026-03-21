use crate::hxposed::error::{NotAllowedReason, NotFoundReason};
use crate::hxposed::responses::HypervisorResponse;
use core::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum HypervisorError {
    Success,
    NotAllowed(NotAllowedReason),
    NotFound(NotFoundReason),
    InvalidParameters(u32),
    NtError(u32),
    TimedOut,
    HvNotLoaded,
    Unknown
}

impl fmt::Debug for HypervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "Success"),
            Self::NotAllowed(reason) => f.debug_tuple("NotAllowed").field(reason).finish(),
            Self::NotFound(reason) => f.debug_tuple("NotFound").field(reason).finish(),
            Self::InvalidParameters(val) => f
                .debug_tuple("InvalidParameters")
                .field(&format_args!("{:#x}", val))
                .finish(),
            Self::TimedOut => write!(f, "TimedOut"),
            Self::NtError(val) => write!(f, "NtError: {:#x}", val),
            Self::HvNotLoaded => write!(f, "HvNotLoaded"),
            Self::Unknown => write!(f, "UnknownError"),
        }
    }
}

impl fmt::Display for HypervisorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "Operation succeeded"),
            Self::NotAllowed(reason) => write!(f, "Access denied: {:?}", reason),
            Self::NotFound(reason) => write!(f, "Resource not found: {:?}", reason),
            Self::InvalidParameters(val) => write!(f, "Invalid parameters provided ({:#x})", val),
            Self::TimedOut => write!(f, "Operation took too long"),
            Self::NtError(val) => write!(f, "Internal NT returned error: {:#x}", val),
            Self::HvNotLoaded => write!(f, "Hypervisor is not loaded"),
            HypervisorError::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl core::error::Error for HypervisorError {}

impl HypervisorError {
    pub fn from_response(response: &HypervisorResponse) -> HypervisorError {
        match response.result.error_code {
            0 => HypervisorError::Success,
            1 => HypervisorError::NotAllowed(NotAllowedReason::from_bits(response.result.error_reason as _)),
            2 => HypervisorError::NotFound(NotFoundReason::from_bits(response.result.error_reason as _)),
            3 => HypervisorError::InvalidParameters(response.result.error_reason as _),
            4 => HypervisorError::NtError(response.result.error_reason as _),
            5 => HypervisorError::TimedOut,
            6 => HypervisorError::HvNotLoaded, // HvNotLoaded is a pseudo error. its returned by vmcall mechanism when RCX is not 2009.
            _ => Self::Unknown,
        }
    }
}
