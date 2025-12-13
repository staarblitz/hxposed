use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use crate::hxposed::responses::security::*;
use crate::services::types::security_fields::TokenPrivilege;
use alloc::boxed::Box;
use core::mem;

pub struct OpenTokenRequest {
    pub addr: u64,
    pub open_type: ObjectOpenType,
}

pub struct CloseTokenRequest {
    pub addr: u64,
}

#[derive(Debug, Default, Clone)]
pub struct GetTokenFieldRequest {
    pub addr: u64,
    pub field: TokenField,
    pub data: *mut u8,
    pub data_len: usize,
}

#[derive(Debug, Default, Clone)]
pub struct SetTokenFieldRequest {
    pub addr: u64,
    pub field: TokenField,
    pub data: *mut u8,
    pub data_len: usize,
}

impl SetTokenFieldRequest {
    pub(crate) fn set_enabled_privileges(addr: u64, new_privileges: &mut TokenPrivilege) -> Self {
        Self {
            addr,
            field: TokenField::EnabledPrivileges,
            data: new_privileges as *mut _ as _,
            data_len: size_of::<TokenPrivilege>(),
        }
    }
}

impl VmcallRequest for SetTokenFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::set_token_field(),
            arg1: self.addr,
            arg2: self.field.clone().into_bits() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1,
            field: TokenField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for GetTokenFieldRequest {
    type Response = GetTokenFieldResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::get_token_field(),
            arg1: self.addr,
            arg2: self.field.clone().into_bits() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1,
            field: TokenField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for CloseTokenRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::close_token(),
            arg1: self.addr,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self { addr: request.arg1 }
    }
}

impl VmcallRequest for OpenTokenRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::open_token(),
            arg1: self.addr,
            arg2: self.open_type.clone().to_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum TokenField {
    #[default]
    Unknown,
    SourceName = 1,
    AccountName = 2,
    Type = 3,
    IntegrityLevelIndex = 4,
    MandatoryPolicy = 5,
    ImpersonationLevel = 6,
    PresentPrivileges = 7,
    EnabledPrivileges = 8,
    EnabledByDefaultPrivileges = 9,
}

impl TokenField {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => TokenField::Unknown,
            1 => TokenField::SourceName,
            2 => TokenField::AccountName,
            3 => TokenField::Type,
            4 => TokenField::IntegrityLevelIndex,
            5 => TokenField::MandatoryPolicy,
            6 => TokenField::ImpersonationLevel,
            7 => TokenField::PresentPrivileges,
            8 => TokenField::EnabledPrivileges,
            9 => TokenField::EnabledByDefaultPrivileges,
            _ => TokenField::Unknown,
        }
    }
}
