use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::services::types::security_fields::{ImpersonationLevel, TokenType};

#[derive(Clone, Default, Debug)]
#[repr(u16)]
pub enum GetTokenFieldResponse {
    #[default]
    Unknown = 0,
    SourceName(u64), // actually a char[8] lol
    AccountName(u16),
    Type(TokenType),
    IntegrityLevelIndex(u32),
    MandatoryPolicy(u32),
    ImpersonationLevel(ImpersonationLevel),
}

impl VmcallResponse for GetTokenFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from_response(raw))
        }

        Ok(match raw.arg1 {
            1 => Self::SourceName(raw.arg2),
            2 => Self::AccountName(raw.arg2 as _),
            3 => Self::Type(TokenType::from_bits(raw.arg2 as _)),
            4 => Self::IntegrityLevelIndex(raw.arg2 as _),
            5 => Self::MandatoryPolicy(raw.arg2 as _),
            6 => Self::ImpersonationLevel(ImpersonationLevel::from_bits(raw.arg2 as _)),
            _ => unreachable!("Developer forgot to implement this one."),
        })
    }

    fn into_raw(self) -> HypervisorResponse {
        let (arg1, arg2, arg3) = match self {
            Self::SourceName(x) => (1, x, 0),
            Self::AccountName(x) => (2, x as _, 0),
            Self::Type(x) => (3, x.into_bits() as _, 0),
            Self::IntegrityLevelIndex(x) => (4, x as _, 0),
            Self::MandatoryPolicy(x) => (5, x as _, 0),
            Self::ImpersonationLevel(x) => (6, x.into_bits() as _, 0),
            _ => unreachable!(),
        };

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetTokenField),
            arg1,
            arg2,
            arg3
        }
    }
}