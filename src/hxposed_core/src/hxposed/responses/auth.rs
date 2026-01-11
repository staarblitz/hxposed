use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::plugins::plugin_perms::PluginPermissions;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct AuthorizationResponse {
    pub permissions: PluginPermissions,
}

impl VmcallResponse for AuthorizationResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            permissions: PluginPermissions::from_bits(raw.arg1).unwrap(),
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::Authorize),
            arg1: self.permissions.bits(),
            ..Default::default()
        }
    }
}
