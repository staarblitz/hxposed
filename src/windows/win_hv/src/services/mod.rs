use crate::plugins::plugin::Plugin;
use crate::services::process_services::*;
use crate::write_response;
use core::ops::BitAnd;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::{HypervisorCall, ServiceParameter};
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::auth::AuthorizationResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::services::async_service::{AsyncInfo, UnsafeAsyncInfo};
use wdk_sys::HANDLE;
use wdk_sys::ntddk::IoGetCurrentProcess;

pub mod process_services;

///
/// # Authorize Plugin
///
/// Authorizes an *existing* plugin.
///
/// ## Warning
/// - This function integrates the plugin with *current* process explicitly.
///
/// ## Returns
///
/// * [`None`] - Plugin was not found in database.
/// * [`Some`] - Plugin was found, authorized with permissions and integrated with the current process.
pub fn authorize_plugin(
    guest: &mut dyn Guest,
    request: AuthorizationRequest,
) -> Option<&'static mut Plugin> {
    let plugin = Plugin::lookup(request.uuid);

    if plugin.is_none() {
        write_response(
            guest,
            HypervisorResponse::not_allowed(NotAllowedReason::PluginNotLoaded),
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
    request: &HypervisorRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) {
    let result = match request.call.func() {
        ServiceFunction::OpenProcess => {
            open_process(guest, OpenProcessRequest::from_raw(request), plugin)
        }
        ServiceFunction::CloseProcess => {
            close_process(guest, CloseProcessRequest::from_raw(request), plugin)
        }
        ServiceFunction::GetProcessField => get_process_field_async(
            guest,
            GetProcessFieldRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::SetProcessField => set_process_field_async(
            guest,
            SetProcessFieldRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::KillProcess => {
            if !request.call.is_async() {
                HypervisorResponse::invalid_params(ServiceParameter::IsAsync)
            } else {
                kill_process_async(
                    guest,
                    KillProcessRequest::from_raw(request),
                    plugin,
                    async_info,
                )
            }
        }
        _ => unreachable!(),
    };

    write_response(guest, result);
}
