#![allow(dead_code)]

use crate::hxposed::TokenObject;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use crate::hxposed::responses::security::*;
use crate::services::types::security_fields::{ImpersonationLevel, TokenPrivilege, TokenType};

pub struct OpenTokenRequest {
    pub token: TokenObject,
    pub open_type: ObjectOpenType,
}

pub struct CloseTokenRequest {
    pub token: TokenObject,
}

#[derive(Debug, Clone)]
pub struct GetTokenFieldRequest {
    pub token: TokenObject,
    pub field: TokenField,
}

#[derive(Debug, Clone)]
pub struct SetTokenFieldRequest {
    pub token: TokenObject,
    pub field: TokenField,
}

impl VmcallRequest for SetTokenFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::set_token_field(),
            arg1: self.token,
            arg2: args.0,
            arg3: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            token: request.arg1,
            field: TokenField::from_raw_enum(request.arg2, request.arg3),
        }
    }
}

impl VmcallRequest for GetTokenFieldRequest {
    type Response = GetTokenFieldResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::get_token_field(),
            arg1: self.token,
            arg2: args.0,
            arg3: args.1,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            token: request.arg1,
            field: TokenField::from_raw_enum(request.arg2, request.arg3),
        }
    }
}

impl VmcallRequest for CloseTokenRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::close_token(),
            arg1: self.token,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            token: request.arg1,
        }
    }
}

impl VmcallRequest for OpenTokenRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::open_token(),
            arg1: self.token,
            arg2: self.open_type.clone().to_bits() as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            token: request.arg1,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenField {
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

impl TokenField {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            TokenField::SourceName(x) => (1, x),
            TokenField::AccountName(x) => (2, x as _),
            TokenField::Type(x) => (3, x.into_bits() as _),
            TokenField::IntegrityLevelIndex(x) => (4, x as _),
            TokenField::MandatoryPolicy(x) => (5, x as _),
            TokenField::ImpersonationLevel(x) => (6, x.into_bits() as _),
            TokenField::EnabledPrivileges(x) => (7, x.bits() as _),
            TokenField::PresentPrivileges(x) => (8, x.bits() as _),
            TokenField::EnabledByDefaultPrivileges(x) => (9, x.bits() as _),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            1 => TokenField::SourceName(value as _),
            2 => TokenField::AccountName(value as _),
            3 => TokenField::Type(TokenType::from_bits(value as _)),
            4 => TokenField::IntegrityLevelIndex(value as _),
            5 => TokenField::MandatoryPolicy(value as _),
            6 => TokenField::ImpersonationLevel(ImpersonationLevel::from_bits(value as _)),
            7 => TokenField::EnabledPrivileges(TokenPrivilege::from_bits_truncate(value as _)),
            8 => TokenField::PresentPrivileges(TokenPrivilege::from_bits_truncate(value as _)),
            9 => TokenField::EnabledByDefaultPrivileges(TokenPrivilege::from_bits_truncate(
                value as _,
            )),
            _ => panic!(),
        }
    }
}
