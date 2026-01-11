use crate::hxposed::call::ServiceParameter;
use crate::hxposed::error::{ErrorSource, InternalErrorCode, NotAllowedReason};
use crate::hxposed::responses::HypervisorResponse;
use alloc::format;
use alloc::string::ToString;
use core::fmt::{Debug, Display, Error, Formatter};

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct HypervisorError {
    pub error_source: ErrorSource,
    pub error_code: u16,
    pub error_reason: u16, // I know I'm missing a big pattern matching potential here.
}

impl Debug for HypervisorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_view(f)?
    }
}

impl Display for HypervisorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.fmt_view(f)?
    }
}
impl HypervisorError {
    pub fn ok() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::Ok as _,
            error_reason: InternalErrorCode::Ok as _,
        }
    }

    pub fn not_found() -> HypervisorError {
        Self {
            error_source: ErrorSource::Hx,
            error_code: InternalErrorCode::NotFound as _,
            error_reason: InternalErrorCode::NotFound as _,
        }
    }

    pub fn is_err(&self) -> bool {
        !(self.error_code == InternalErrorCode::Ok as _ && self.error_source == ErrorSource::Hx)
    }

    pub fn from_response(response: HypervisorResponse) -> HypervisorError {
        Self {
            error_source: response.result.error_source(),
            error_code: response.result.error_code() as _,
            error_reason: response.arg1 as _,
        }
    }

    fn fmt_view(&self, f: &mut Formatter) -> Result<Result<(), Error>, Error> {
        writeln!(f, "Hypervisor returned an error.")?;

        let source = match self.error_source {
            ErrorSource::Hx => "HxPosed",
            ErrorSource::Nt => "Windows Kernel",
            ErrorSource::Hv => "CPU Internal",
        };
        writeln!(f, "Error source: {source}")?;

        match self.error_source {
            ErrorSource::Hx => {
                let code = InternalErrorCode::from_bits(self.error_code);
                writeln!(f, "Error code: {:?}", code)?;
            }
            _ => writeln!(f, "Error code: {:x}", self.error_code)?,
        }

        let reason_string = match self.error_source {
            ErrorSource::Hx => match InternalErrorCode::from_bits(self.error_code) {
                InternalErrorCode::NotAllowed => {
                    format!("{:?}", NotAllowedReason::from_bits(self.error_reason as _))
                }
                InternalErrorCode::InvalidParams => {
                    format!("{:?}", ServiceParameter::from_bits(self.error_reason as _))
                }
                InternalErrorCode::NotFound => format!(
                    "Object {:?} not found",
                    NotAllowedReason::from_bits(self.error_reason as _)
                ),
                _ => "None".to_string(),
            },
            _ => format!("Error reason {:?}", self.error_reason),
        };

        writeln!(f, "Error reason: {reason_string}")?;
        Ok(Ok(()))
    }
}
