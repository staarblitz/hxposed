use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::notify::ObjectEventType;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::ObjectType;

#[derive(Debug, Clone)]
pub struct NotifyEventResponse {
    pub object_type: ObjectType,
    pub event_type: ObjectEventType,
}

impl VmcallResponse for NotifyEventResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            object_type: ObjectType::from_raw(raw.arg1, raw.arg2),
            event_type: ObjectEventType::from_bits(raw.arg3 as _),
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.object_type.into_raw();

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::NotifyEvent),
            arg1: args.0,
            arg2: args.1,
            arg3: self.event_type.to_bits() as _,

            ..Default::default()
        }
    }
}
