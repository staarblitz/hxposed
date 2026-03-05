use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use crate::hxposed::responses::thread::*;
use crate::hxposed::ThreadObject;

#[derive(Clone, Default, Debug)]
pub struct OpenThreadRequest {
    pub pid: u32,
    pub tid: u32,
    pub open_type: ObjectOpenType,
}

#[derive(Clone, Default, Debug)]
pub struct CloseThreadRequest {
    pub thread: ThreadObject,
    pub open_type: ObjectOpenType,
}

#[derive(Debug, Clone)]
pub struct GetThreadFieldRequest {
    pub thread: ThreadObject,
    pub field: ThreadField,
}

#[derive(Debug)]
pub struct SetThreadFieldRequest {
    pub thread: ThreadObject,
    pub field: ThreadField,
}

impl VmcallRequest for GetThreadFieldRequest {
    type Response = GetThreadFieldResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::get_thread_field(),
            arg1: self.thread as _,
            arg2: args.0,
            arg3: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            thread: request.arg1 as _,
            field: ThreadField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl VmcallRequest for SetThreadFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.field.into_raw_enum();
        HypervisorRequest {
            call: HypervisorCall::set_thread_field(),
            arg1: self.thread as _,
            arg2: args.0,
            arg3: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            thread: request.arg1 as _,
            field: ThreadField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl VmcallRequest for CloseThreadRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::close_thread(),
            arg1: self.thread.clone() as _,
            arg2: self.open_type.clone().to_bits() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            thread: request.arg1 as _,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for OpenThreadRequest {
    type Response = OpenObjectResponse; // it works. that's all I can say

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::open_thread(),
            arg1: self.pid as _,
            arg2: self.tid as _,
            arg3: self.open_type.clone().to_bits() as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            pid: request.arg1 as _,
            tid: request.arg2 as _,
            open_type: ObjectOpenType::from_bits(request.arg3 as _),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThreadField {
    ActiveImpersonationInfo(bool),
    AdjustedClientToken(u64),
}

impl ThreadField {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            ThreadField::ActiveImpersonationInfo(x) => (1, x as _),
            ThreadField::AdjustedClientToken(x) => (2, x as _),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            1 => ThreadField::ActiveImpersonationInfo(value == 1),
            2 => ThreadField::AdjustedClientToken(value as _),
            _ => panic!()
        }
    }
}

#[derive(Clone, Default, Debug)]
pub enum ThreadContextOperation {
    #[default]
    Set,
    Get,
}

impl ThreadContextOperation {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            1 => Self::Get,
            _ => Self::Set,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub enum SuspendResumeThreadOperation {
    #[default]
    Suspend,
    Resume,
    Freeze,
}

impl SuspendResumeThreadOperation {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            1 => Self::Resume,
            2 => Self::Suspend,
            3 => Self::Freeze,
            _ => Self::Suspend,
        }
    }
}
