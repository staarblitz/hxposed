use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::{ObjectOpenType, ProcessField};
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
    pub addr: u64,
    pub operation: SuspendResumeThreadOperation,
}

#[derive(Clone, Default, Debug)]
pub struct KillThreadRequest {
    pub addr: u64,
    pub exit_code: u32,
}

#[derive(Clone, Default, Debug)]
pub struct GetSetThreadContextRequest {
    pub addr: u64,
    pub operation: ThreadContextOperation,
    pub data: *mut u8,
    pub data_len: usize,
}

#[derive(Default, Debug, Clone)]
pub struct GetThreadFieldRequest {
    pub addr: u64,
    pub field: ThreadField,
    pub data: *mut u8,
    pub data_len: usize,
}

#[derive(Default, Debug)]
pub struct SetThreadFieldRequest {
    pub addr: u64,
    pub field: ThreadField,
    pub data: *mut u8,
    pub data_len: usize,
}

impl VmcallRequest for GetThreadFieldRequest {
    type Response = GetThreadFieldResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::get_thread_field(),
            arg1: self.addr as _,
            arg2: self.field.clone() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,
            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1 as _,
            field: ThreadField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as *mut u8,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for SetThreadFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::set_thread_field(),
            arg1: self.addr as _,
            arg2: self.field.clone() as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1 as _,
            field: ThreadField::from_bits(request.arg2 as _),
            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
        }
    }
}

impl VmcallRequest for GetSetThreadContextRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::get_set_thread_context(),
            arg1: self.addr as _,
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
            addr: request.arg1 as _,
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
            arg1: self.addr as _,
            arg2: self.exit_code as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(raw: &HypervisorRequest) -> Self {
        Self {
            addr: raw.arg1 as _,
            exit_code: raw.arg2 as _,
        }
    }
}

impl VmcallRequest for SuspendResumeThreadRequest {
    type Response = SuspendThreadResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::suspend_resume_thread(),
            arg1: self.addr as _,
            arg2: self.operation.clone().into_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            addr: request.arg1 as _,
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
pub enum ThreadField {
    #[default]
    Unknown = 0,
    ActiveImpersonationInfo = 1,
    AdjustedClientToken = 2,
}

impl ThreadField {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => ThreadField::Unknown,
            1 => ThreadField::ActiveImpersonationInfo,
            2 => ThreadField::AdjustedClientToken,
            _ => unreachable!(),
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
