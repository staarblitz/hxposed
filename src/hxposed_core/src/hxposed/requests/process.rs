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

#[derive(Default, Debug, Clone)]
pub struct GetProcessFieldRequest {
    pub process: ProcessObject,
    pub field: ProcessField,
    pub data: usize,
    pub data_len: usize,
}

#[derive(Default, Debug)]
pub struct SetProcessFieldRequest {
    pub process: ProcessObject,
    pub field: ProcessField,
    pub data: usize,
    pub data_len: usize,
}

///TODO: Maybe merge with [GetProcessFieldRequest]?
#[derive(Default, Debug)]
pub struct GetProcessThreadsRequest {
    pub process: ProcessObject,
    pub data: usize,
    pub data_len: usize,
}

impl VmcallRequest for GetProcessThreadsRequest {
    type Response = GetProcessThreadsResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::get_process_threads(),
            arg1: self.process as _,
            arg2: self.data as _,
            arg3: self.data_len as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            data: request.arg2 as _,
            data_len: request.arg3 as _,
        }
    }
}

impl VmcallRequest for OpenProcessRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: match self.open_type.clone() {
                ObjectOpenType::Handle => HypervisorCall::open_process().with_is_async(true),
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
        HypervisorRequest {
            call: HypervisorCall::get_process_field(),
            arg1: self.process as _,
            arg2: self.field.clone() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            field: ProcessField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as usize,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for SetProcessFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::set_process_field(),
            arg1: self.process as _,
            arg2: self.field.clone() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            field: ProcessField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl SetProcessFieldRequest {
    pub(crate) fn set_protection(addr: u64, new_protection: &mut ProcessProtection) -> Self {
        Self {
            process: addr,
            field: ProcessField::Protection,
            data: new_protection as *mut _ as _,
            data_len: size_of::<ProcessProtection>(), // 1 byte
        }
    }

    pub(crate) fn set_signature_levels(addr: u64, new_levels: &mut ProcessSignatureLevels) -> Self {
        Self {
            process: addr,
            field: ProcessField::Signers,
            data: new_levels as *mut _ as _,
            data_len: size_of::<ProcessSignatureLevels>(),
        }
    }

    pub(crate) fn set_mitigation_options(addr: u64, new_options: &mut MitigationOptions) -> Self {
        Self {
            process: addr,
            field: ProcessField::MitigationFlags,
            data: new_options as *mut _ as _,
            data_len: size_of::<MitigationOptions>(),
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ProcessField {
    #[default]
    Unknown,
    NtPath = 1,
    Protection = 2,
    Signers = 3,
    MitigationFlags = 4,
    Token = 5,
}

impl ProcessField {
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            1 => Self::NtPath,
            2 => Self::Protection,
            3 => Self::Signers,
            4 => Self::MitigationFlags,
            5 => Self::Token,
            _ => Self::Unknown,
        }
    }

    pub const fn into_bits(self) -> u16 {
        self as _
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
