use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use crate::hxposed::responses::thread::*;
use alloc::boxed::Box;
use core::mem;

#[derive(Clone, Default, Debug)]
pub struct OpenThreadRequest {
    pub pid: u32,
    pub tid: u32,
    pub open_type: ObjectOpenType,
}

#[derive(Clone, Default, Debug)]
pub struct CloseThreadRequest {
    pub addr: u64,
    pub open_type: ObjectOpenType,
}

#[derive(Clone, Default, Debug)]
pub struct SuspendResumeThreadRequest {
    pub id: u32,
    pub operation: SuspendResumeThreadOperation,
}

#[derive(Clone, Default, Debug)]
pub struct KillThreadRequest {
    pub id: u32,
    pub exit_code: u32,
}

#[derive(Clone,Default,Debug)]
pub struct GetSetThreadContextRequest {
    pub id: u32,
    pub operation: ThreadContextOperation,
    pub data: *mut u8,
    pub data_len: usize
}

impl VmcallRequest for GetSetThreadContextRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::get_set_thread_context(),
            arg1: self.id as _,
            arg2: self.operation.clone().into_bits() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
            operation: ThreadContextOperation::from_bits(request.arg2 as _),
            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for KillThreadRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::kill_thread(),
            arg1: self.id as _,
            arg2: self.exit_code as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(raw: &HypervisorRequest) -> Self {
        Self {
            id: raw.arg1 as _,
            exit_code: raw.arg2 as _,
        }
    }
}

impl VmcallRequest for SuspendResumeThreadRequest {
    type Response = SuspendThreadResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::suspend_resume_thread(),
            arg1: self.id as _,
            arg2: self.operation.clone().into_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
            operation: SuspendResumeThreadOperation::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for CloseThreadRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::close_thread(),
            arg1: self.addr.clone() as _,
            arg2: self.open_type.clone().to_bits() as _,
            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1 as _,
            open_type: ObjectOpenType::from_bits(request.arg2 as _),
        }
    }
}

impl VmcallRequest for OpenThreadRequest {
    type Response = OpenObjectResponse; // it works. that's all I can say

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: match self.open_type.clone() {
                ObjectOpenType::Handle => HypervisorCall::open_thread().with_is_async(true),
                ObjectOpenType::Hypervisor => HypervisorCall::open_thread(),
            },
            arg1: self.pid as _,
            arg2: self.tid as _,
            arg3: self.open_type.clone().to_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            pid: request.arg1 as _,
            tid: request.arg2 as _,
            open_type: ObjectOpenType::from_bits(request.arg3 as _),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub enum ThreadContextOperation{
    #[default]
    Set,
    Get
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
