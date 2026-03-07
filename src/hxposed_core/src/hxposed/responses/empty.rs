use crate::hxposed::call::HypervisorResult;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::ObjectType;

///TODO: Replace with ()?
#[derive(Clone, Debug)]
pub struct EmptyResponse;

// why is this here???
#[derive(Clone, Debug)]
pub struct OpenObjectResponse {
    pub object: ObjectType,
}

impl VmcallResponse for OpenObjectResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            object: ObjectType::from_raw(raw.arg1, raw.arg2),
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let object = ObjectType::into_raw(self.object);

        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: object.0,
            arg2: object.1,
            ..Default::default()
        }
    }
}

impl EmptyResponse {
    pub fn default() -> HypervisorResponse {
        HypervisorResponse::default()
    }
}

impl VmcallResponse for EmptyResponse {
    fn from_raw(_raw: HypervisorResponse) -> Self {
        EmptyResponse
    }

    fn into_raw(self) -> HypervisorResponse {
        panic!("Use with_service instead.")
    }
}
