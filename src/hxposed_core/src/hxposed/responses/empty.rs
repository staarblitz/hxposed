use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub struct EmptyResponse;

impl EmptyResponse{
    pub fn with_service(service_function: ServiceFunction) -> HypervisorResponse {
        HypervisorResponse{
            result: HypervisorResult::ok(service_function),
            ..Default::default() // this is actually a very cool feature.
        }
    }
}

impl VmcallResponse for EmptyResponse {
    fn from_raw(raw: HypervisorResponse) -> Result<Self, HypervisorError> {
        Ok(EmptyResponse {})
    }

    fn into_raw(self) -> HypervisorResponse {
        panic!("Use with_service instead.")
    }
}