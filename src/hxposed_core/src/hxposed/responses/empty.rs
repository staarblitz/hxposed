use crate::hxposed::call::HypervisorResult;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::ObjectType;

///TODO: Replace with ()?
#[derive(Clone, Debug)]
pub struct EmptyResponse;

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
