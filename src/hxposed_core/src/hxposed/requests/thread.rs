use crate::hxposed::call::HxCall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::empty::{EmptyResponse};
use crate::hxposed::responses::OpenObjectResponse;
use crate::hxposed::responses::thread::*;
use crate::hxposed::ThreadObject;

#[derive(Clone, Default, Debug)]
pub struct OpenThreadRequest {
    pub tid: u64,
}

#[derive(Clone, Default, Debug)]
pub struct CloseThreadRequest {
    pub thread: ThreadObject,
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

impl SyscallRequest for GetThreadFieldRequest {
    type Response = GetThreadFieldResponse;

    fn into_raw(self) -> HxRequest {
        let args = self.field.into_raw_enum();
        HxRequest {
            call: HxCall::get_thread_field(),
            arg1: self.thread as _,
            arg2: args.0,
            arg3: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            thread: request.arg1 as _,
            field: ThreadField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl SyscallRequest for SetThreadFieldRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HxRequest {
        let args = self.field.into_raw_enum();
        HxRequest {
            call: HxCall::set_thread_field(),
            arg1: self.thread as _,
            arg2: args.0,
            arg3: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            thread: request.arg1 as _,
            field: ThreadField::from_raw_enum(request.arg2, request.arg3)
        }
    }
}

impl SyscallRequest for CloseThreadRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::close_thread(),
            arg1: self.thread.clone() as _,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            thread: request.arg1 as _,
        }
    }
}

impl SyscallRequest for OpenThreadRequest {
    type Response = OpenObjectResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::open_thread(),
            arg1: self.tid as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            tid: request.arg1 as _,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ThreadField {
    ActiveImpersonationInfo(bool),
    AdjustedClientToken(u64),
    Unknown
}

impl ThreadField {
    pub fn into_raw_enum(self) -> (u64, u64) {
        match self {
            ThreadField::ActiveImpersonationInfo(x) => (1, x as _),
            ThreadField::AdjustedClientToken(x) => (2, x as _),
            ThreadField::Unknown => (0, 0),
        }
    }

    pub fn from_raw_enum(object: u64, value: u64) -> Self {
        match object {
            1 => ThreadField::ActiveImpersonationInfo(value == 1),
            2 => ThreadField::AdjustedClientToken(value as _),
            _ => ThreadField::Unknown
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
