use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::notify::ObjectState;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::{CallbackObject, ObjectType};

#[derive(Debug, Clone)]
pub struct RegisterNotifyHandlerResponse {
    pub callback: CallbackObject,
}

#[derive(Debug, Clone)]
pub struct AwaitNotificationResponse {
    pub object_type: ObjectType,
    pub object_state: ObjectState,
}

impl VmcallResponse for AwaitNotificationResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            object_type: ObjectType::from_raw(raw.arg1, raw.arg2),
            object_state: ObjectState::from_bits(raw.arg3 as _),
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.object_type.into_raw();

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::AwaitNotifyEvent),
            arg1: args.0,
            arg2: args.1,
            arg3: self.object_state.to_bits() as _,
            ..Default::default()
        }
    }
}

impl VmcallResponse for RegisterNotifyHandlerResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self { callback: raw.arg1 }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::RegisterNotifyEvent),
            arg1: self.callback,

            ..Default::default()
        }
    }
}
