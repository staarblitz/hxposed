use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::notify::{AwaitNotificationResponse, RegisterNotifyHandlerResponse};
use crate::hxposed::{CallbackObject, ObjectType};

pub struct RegisterNotifyHandlerRequest {
    pub target_object: ObjectType,
}

pub struct UnregisterNotifyHandlerRequest {
    pub callback: CallbackObject,
}

pub struct AwaitNotificationRequest {
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

impl VmcallRequest for AwaitNotificationRequest {
    type Response = AwaitNotificationResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::await_notify_event(),
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
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            target_object: ObjectType::from_raw(request.arg1, request.arg2)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum ObjectState {
    Created,
    Modified,
    Deleted,
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
