use alloc::boxed::Box;
use core::arch::x86_64::__m128i;
use core::mem;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::OpenObjectResponse;
use crate::hxposed::responses::security::*;

pub struct OpenTokenRequest {
    pub addr: u64
}

#[derive(Debug, Default, Clone)]
pub struct GetTokenFieldRequest {
    pub addr: u64,
    pub field: TokenField,
    pub data: *mut u8,
    pub data_len: usize
}

impl VmcallRequest for GetTokenFieldRequest {
    type Response = GetTokenFieldResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest{
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

impl VmcallRequest for OpenTokenRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::open_token(),
            arg1: self.addr,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1
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
            _ => TokenField::Unknown,
        }
    }
}