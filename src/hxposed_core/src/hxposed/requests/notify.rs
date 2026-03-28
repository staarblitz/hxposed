use crate::hxposed::call::HxCall;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::notify::{RegisterNotifyHandlerResponse};
use crate::hxposed::{CallbackObject, ObjectType};

pub struct RegisterNotifyHandlerRequest {
    pub target_object: ObjectType,
    pub event_handle: u64,
    pub memory: u64
}

pub struct UnregisterNotifyHandlerRequest {
    pub callback: CallbackObject,
}

impl SyscallRequest for UnregisterNotifyHandlerRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::unregister_notify_event(),
            arg1: self.callback,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            callback: request.arg1,
        }
    }
}

impl SyscallRequest for RegisterNotifyHandlerRequest {
    type Response = RegisterNotifyHandlerResponse;

    fn into_raw(self) -> HxRequest {
        let args = self.target_object.into_raw();
        HxRequest {
            call: HxCall::register_notify_event(),
            arg1: args.0,
            arg2: self.event_handle,
            arg3: self.memory,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            target_object: ObjectType::from_raw(request.arg1, 0),
            memory: request.arg3,
            event_handle: request.arg2
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
#[repr(u8)]
pub enum ObjectState {
    Created = 0,
    Modified = 1,
    Deleted = 2,
    #[default]
    Unknown = u8::MAX
}

impl ObjectState {
    pub const fn from_bits(bits: u8) -> Self {
        if bits >= 3 {
            unsafe {
                core::mem::transmute(bits)
            }
        } else {
            ObjectState::Unknown
        }
    }

    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
