use crate::hxposed::call::HxResult;
use crate::hxposed::responses::{HxResponse, SyscallResponse};
use crate::hxposed::ObjectType;

///TODO: Replace with ()?
#[derive(Clone, Debug)]
pub struct EmptyResponse;

impl EmptyResponse {
    pub fn default() -> HxResponse {
        HxResponse::default()
    }
}

impl SyscallResponse for EmptyResponse {
    fn from_raw(_raw: HxResponse) -> Self {
        EmptyResponse
    }

    fn into_raw(self) -> HxResponse {
        panic!("Use with_service instead.")
    }
}
