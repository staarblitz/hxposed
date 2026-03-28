#![allow(dead_code)]

use crate::hxposed::call::HxCall;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::empty::{EmptyResponse};
use crate::hxposed::responses::process::*;
use crate::hxposed::ProcessObject;
use crate::hxposed::responses::OpenObjectResponse;
use crate::services::types::process_fields::*;

#[derive(Clone, Default, Debug)]
pub struct OpenProcessRequest {
    pub process_id: u64,
}

#[derive(Clone, Default, Debug)]
pub struct CloseProcessRequest {
    pub process: ProcessObject,
}

#[derive(Clone, Default, Debug)]
pub struct KillProcessRequest {
    pub process: ProcessObject,
    pub exit_code: u32,
}

#[derive(Debug, Clone)]
pub struct GetProcessFieldRequest {
    pub process: ProcessObject,
    pub field: ProcessField,
}

#[derive(Debug)]
pub struct SetProcessFieldRequest {
    pub process: ProcessObject,
    pub field: ProcessField,
}

impl SyscallRequest for OpenProcessRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::open_process(),
            arg1: self.process_id as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            process_id: request.arg1 as _,
        }
    }
}

impl SyscallRequest for CloseProcessRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::close_process(),
            arg1: self.process as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            process: request.arg1 as _,
        }
    }
}

impl SyscallRequest for GetProcessFieldRequest {
    type Response = GetProcessFieldResponse;

    fn into_raw(self) -> HxRequest {
        let objs = self.field.into_raw_enum();
        HxRequest {
            call: HxCall::get_process_field(),
            arg1: self.process as _,
            arg2: objs.0,
            arg3: objs.1,

            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            process: request.arg1 as _,
            field: ProcessField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl SyscallRequest for SetProcessFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HxRequest {
        let objs = self.field.into_raw_enum();
        HxRequest {
            call: HxCall::set_process_field(),
            arg1: self.process as _,
            arg2: objs.0,
            arg3: objs.1,

            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            process: request.arg1 as _,
            field: ProcessField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ProcessField {
    NtPath(u64),
    Protection(ProcessProtection),
    Signers(ProcessSignatureLevels),
    MitigationFlags(MitigationOptions),
    Token(u64),
    Threads(u64),
    DirectoryTableBase(u64),
    UserDirectoryTableBase(u64),
    Unknown
}

impl ProcessField {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            ProcessField::NtPath(x) => (1, x),
            ProcessField::Protection(x) => (2, x.into_bits() as _),
            ProcessField::Signers(x) => (3, x.into_bits() as _),
            ProcessField::MitigationFlags(x) => (4, x.into_bits() as _),
            ProcessField::Token(x) => (5, x),
            ProcessField::Threads(x) => (6, x),
            ProcessField::DirectoryTableBase(x) => (7, x),
            ProcessField::UserDirectoryTableBase(x) => (8, x),
            ProcessField::Unknown => (0, 0),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            1 => ProcessField::NtPath(value),
            2 => ProcessField::Protection(ProcessProtection::from_bits(value as _)),
            3 => ProcessField::Signers(ProcessSignatureLevels::from_bits(value as _)),
            4 => ProcessField::MitigationFlags(MitigationOptions::from_bits(value as _)),
            5 => ProcessField::Token(value),
            6 => ProcessField::Threads(value),
            7 => ProcessField::DirectoryTableBase(value),
            8 => ProcessField::UserDirectoryTableBase(value),
            _ => ProcessField::Unknown
        }
    }
}


//TODO: move this
#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ObjectOpenType {
    #[default]
    Handle = 0,
    Hypervisor = 1,
}

impl ObjectOpenType {
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => Self::Handle,
            _ => Self::Hypervisor,
        }
    }

    pub const fn to_bits(self) -> u16 {
        self as u16
    }
}
