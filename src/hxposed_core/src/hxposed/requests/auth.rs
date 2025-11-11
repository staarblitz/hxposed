use uuid::Uuid;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::process::OpenProcessRequest;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::auth::AuthorizationResponse;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct AuthorizationRequest {
    pub uuid: Uuid
}

impl VmcallRequest for AuthorizationRequest {
    type Response = AuthorizationResponse;

    fn into_raw(self) -> HypervisorRequest {
        let uuid = self.uuid.as_u64_pair();
        HypervisorRequest{
            call: HypervisorCall::auth(),
            arg1: uuid.0,
            arg2: uuid.1,
            arg3: 0
        }
    }
}