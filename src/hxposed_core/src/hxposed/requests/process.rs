#![allow(dead_code)]

use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use crate::hxposed::responses::process::*;
use crate::hxposed::ProcessObject;
use crate::services::types::process_fields::*;

#[derive(Clone, Default, Debug)]
pub struct OpenProcessRequest {
    pub process_id: u32,
    pub open_type: ObjectOpenType,
}

#[derive(Clone, Default, Debug)]
pub struct CloseProcessRequest {
    pub process: ProcessObject,
    pub open_type: ObjectOpenType,
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

impl VmcallRequest for OpenProcessRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: match self.open_type.clone() {
                ObjectOpenType::Handle => HypervisorCall::open_process(),
                ObjectOpenType::Hypervisor => HypervisorCall::open_process(),
            },
            arg1: self.process_id as _,
            arg2: self.open_type.clone().to_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process_id: request.arg1 as _,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for CloseProcessRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::close_process(),
            arg1: self.process as _,
            arg2: self.open_type.clone().to_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for KillProcessRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::kill_process(),
            arg1: self.process as _,
            arg2: self.exit_code as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            exit_code: request.arg2 as _,
        }
    }
}

impl VmcallRequest for GetProcessFieldRequest {
    type Response = GetProcessFieldResponse;

    fn into_raw(self) -> HypervisorRequest {
        let objs = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::get_process_field(),
            arg1: self.process as _,
            arg2: objs.0,
            arg3: objs.1,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            field: ProcessField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl VmcallRequest for SetProcessFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        let objs = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::set_process_field(),
            arg1: self.process as _,
            arg2: objs.0,
            arg3: objs.1,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
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
    Threads(u64)
}

impl ProcessField {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            ProcessField::NtPath(x) => (1, x),
            ProcessField::Protection(x) => (2, x.into_bits() as _),
            ProcessField::Signers(x) => (3, x.into_bits() as _),
            ProcessField::MitigationFlags(x) => (4, x.into_bits() as _),
            ProcessField::Token(x) => (5, x),
            ProcessField::Threads(x) => (6, x)
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
            _ => panic!("Invalid  object id: {}", object)
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
