use alloc::boxed::Box;
use core::mem;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::auth::AuthorizationResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct AuthorizationRequest {
    pub uuid: Uuid,
    pub permissions: PluginPermissions,
}

impl AuthorizationRequest {
    pub fn new(uuid: Uuid, permissions: PluginPermissions) -> Self {
        Self { uuid, permissions }
    }
}

impl VmcallRequest for AuthorizationRequest {
    type Response = AuthorizationResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let uuid = self.uuid.as_u64_pair();
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::auth(),
            arg1: uuid.0,
            arg2: uuid.1,
            arg3: self.permissions.bits(),
            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            uuid: Uuid::from_u64_pair(request.arg1, request.arg2),
            permissions: PluginPermissions::from_bits(request.arg3).unwrap(),
        }
    }
}
