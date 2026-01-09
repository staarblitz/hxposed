use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::ObjectType;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone, Debug)]
pub struct EmptyResponse;

#[derive(Clone, Debug)]
pub struct OpenObjectResponse {
    pub object: ObjectType,
}

impl VmcallResponse for OpenObjectResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            return Err(HypervisorError::from_response(raw));
        }

        Ok(Self { object: ObjectType::from_raw(raw.arg1, raw.arg2) })
    }

    fn into_raw(self) -> HypervisorResponse {
        let object = ObjectType::into_raw(self.object);

        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::OpenProcess),
            arg1: object.0,
            arg2: object.1,
            ..Default::default()
        }
    }
}

impl EmptyResponse {
    pub fn with_service(service_function: ServiceFunction) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(service_function),
            ..Default::default() // this is actually a very cool feature.
        }
    }
}

impl VmcallResponse for EmptyResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        if raw.result.is_error() {
            Err(HypervisorError::from_response(raw))
        } else {
            Ok(EmptyResponse)
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        panic!("Use with_service instead.")
    }
}
