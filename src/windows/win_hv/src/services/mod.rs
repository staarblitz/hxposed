use crate::plugins::plugin::Plugin;
use crate::services::process_services::*;
use crate::write_response;
use core::ops::BitAnd;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::VmcallRequest;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::process::{CloseProcessRequest, OpenProcessRequest};
use hxposed_core::hxposed::responses::auth::AuthorizationResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use wdk_sys::ntddk::IoGetCurrentProcess;

pub mod process_services;

///
/// # Authorize Plugin
///
/// Authorizes an *existing* plugin.
///
/// ## Returns
///
/// Returns [None] if plugin was not found in database. Returns [Plugin] if authorization was ok.
pub fn authorize_plugin(
    guest: &mut dyn Guest,
    request: AuthorizationRequest,
) -> Option<&'static mut Plugin> {
    let plugin = Plugin::lookup(request.uuid);

    if plugin.is_none() {
        write_response(
            guest,
            HypervisorResponse::not_allowed(
                NotAllowedReason::PluginNotLoaded,
                PluginPermissions::empty(),
            ),
        );
        return None;
    }

    let plugin = plugin.unwrap();
    let permissions = plugin.permissions.bitand(request.permissions);

    plugin.integrate(unsafe { IoGetCurrentProcess() }, permissions);

    write_response(guest, AuthorizationResponse { permissions }.into_raw());

    Some(plugin)
}

///
/// # Handle Process Services
///
/// Dispatches the process service request to [process_services].
///
pub fn handle_process_services(
    guest: &mut dyn Guest,
    call: HypervisorCall,
    args: (u64, u64, u64),
    plugin: &'static mut Plugin,
) {
    let result = match call.func() {
        ServiceFunction::OpenProcess => {
            open_process(guest, OpenProcessRequest::from_raw(call, args), plugin)
        }
        ServiceFunction::CloseProcess => {
            close_process(guest, CloseProcessRequest::from_raw(call, args), plugin)
        }
        _ => unreachable!(),
    };

    write_response(guest, result);
}