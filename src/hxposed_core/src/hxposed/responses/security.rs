use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::services::types::security_fields::{ImpersonationLevel, TokenPrivilege, TokenType};

#[derive(Clone, Debug)]
#[repr(u16)]
pub enum GetTokenFieldResponse {
    SourceName(u64), // actually a char[8] lol
    AccountName(u16),
    Type(TokenType),
    IntegrityLevelIndex(u32),
    MandatoryPolicy(u32),
    ImpersonationLevel(ImpersonationLevel),
    EnabledPrivileges(TokenPrivilege),
    PresentPrivileges(TokenPrivilege),
    EnabledByDefaultPrivileges(TokenPrivilege),
}

impl GetTokenFieldResponse {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            GetTokenFieldResponse::SourceName(value) => (1, value),
            GetTokenFieldResponse::AccountName(value) => (2, value as _),
            GetTokenFieldResponse::Type(token_type) => (3, token_type as _),
            GetTokenFieldResponse::IntegrityLevelIndex(index) => (4, index as _),
            GetTokenFieldResponse::MandatoryPolicy(policy) => (5, policy as _),
            GetTokenFieldResponse::ImpersonationLevel(index) => (6, index as _),
            GetTokenFieldResponse::EnabledPrivileges(privs) => (7, privs.bits()),
            GetTokenFieldResponse::PresentPrivileges(privs) => (8, privs.bits()),
            GetTokenFieldResponse::EnabledByDefaultPrivileges(privs) => (9, privs.bits()),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> GetTokenFieldResponse {
        match object {
            1 => GetTokenFieldResponse::SourceName(value),
            2 => GetTokenFieldResponse::AccountName(value as _),
            3 => GetTokenFieldResponse::Type(TokenType::from_bits(value as _)),
            4 => GetTokenFieldResponse::IntegrityLevelIndex(value as _),
            5 => GetTokenFieldResponse::MandatoryPolicy(value as _),
            6 => {
                GetTokenFieldResponse::ImpersonationLevel(ImpersonationLevel::from_bits(value as _))
            }
            7 => {
                GetTokenFieldResponse::EnabledPrivileges(TokenPrivilege::from_bits_truncate(value))
            }
            8 => {
                GetTokenFieldResponse::PresentPrivileges(TokenPrivilege::from_bits_truncate(value))
            }
            9 => GetTokenFieldResponse::EnabledByDefaultPrivileges(
                TokenPrivilege::from_bits_truncate(value),
            ),
            _ => panic!("Invalid object id: {}", object),
        }
    }
}

impl VmcallResponse for GetTokenFieldResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from_response(raw));
        }

        Ok(GetTokenFieldResponse::from_raw_enum(raw.arg1, raw.arg2))
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.into_raw_enum();

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetTokenField),
            arg1: args.0,
            arg2: args.1,
            arg3: 0,
        }
    }
}
