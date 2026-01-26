use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::notify::ObjectState;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::{CallbackObject, ObjectType};

pub const CALLBACK_RESPONSE_RESERVED_OFFSET: u64 = 0;

#[derive(Debug, Clone)]
pub struct RegisterNotifyHandlerResponse {
    pub callback: CallbackObject,
}

#[derive(Debug, Clone)]
#[repr(C)]
// well, we cannot use the ObjectType enum since rust "cannot guarantee" its stable across 2 binaries.
// correct me if im wrong
pub struct CallbackInformation {
    pub object_type: u64,
    pub object_value: u64,
    pub object_state: ObjectState,
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
