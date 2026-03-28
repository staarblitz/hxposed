use crate::hxposed::call::HxResult;
use crate::hxposed::requests::notify::ObjectState;
use crate::hxposed::responses::{HxResponse, SyscallResponse};
use crate::hxposed::CallbackObject;

pub const CALLBACK_RESPONSE_RESERVED_OFFSET: u64 = 0;

#[derive(Debug, Clone)]
pub struct RegisterNotifyHandlerResponse {
    pub callback: CallbackObject,
}

#[derive(Debug, Clone, Default)]
#[repr(C)]
// well, we cannot use the ObjectType enum since rust "cannot guarantee" its stable across 2 binaries.
// correct me if im wrong
pub struct CallbackInformation {
    pub object_type: u64,
    pub object_value: u64,
    pub object_state: ObjectState,
}

impl SyscallResponse for RegisterNotifyHandlerResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self { callback: raw.arg1 }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.callback,

            ..Default::default()
        }
    }
}
