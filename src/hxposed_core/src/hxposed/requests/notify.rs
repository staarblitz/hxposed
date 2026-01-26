use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::notify::{RegisterNotifyHandlerResponse};
use crate::hxposed::{CallbackObject, ObjectType};

pub struct RegisterNotifyHandlerRequest {
    pub target_object: ObjectType,
    pub event_handle: u64
}

pub struct UnregisterNotifyHandlerRequest {
    pub callback: CallbackObject,
}

impl VmcallRequest for UnregisterNotifyHandlerRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::unregister_notify_event(),
            arg1: self.callback,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            callback: request.arg1,
        }
    }
}

impl VmcallRequest for RegisterNotifyHandlerRequest {
    type Response = RegisterNotifyHandlerResponse;

    fn into_raw(self) -> HypervisorRequest {
        let args = self.target_object.into_raw();
        HypervisorRequest {
            call: HypervisorCall::register_notify_event(),
            arg1: args.0,
            arg2: args.1,
            arg3: self.event_handle,
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            target_object: ObjectType::from_raw(request.arg1, request.arg2),
            event_handle: request.arg3
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[repr(C)]
pub enum ObjectState {
    Created = 0,
    Modified = 1,
    Deleted = 2,
}

impl ObjectState {
    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Created,
            1 => Self::Modified,
            2 => Self::Deleted,
            _ => unreachable!(),
        }
    }

    pub const fn to_bits(self) -> u8 {
        self as u8
    }
}
