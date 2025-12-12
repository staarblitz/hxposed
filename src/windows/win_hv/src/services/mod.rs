use crate::plugins::plugin::Plugin;
use crate::plugins::PluginTable;
use crate::services::memory_services::*;
use crate::services::process_services::*;
use crate::services::security_services::*;
use crate::services::thread_services::*;
use crate::write_response;
use core::ops::BitAnd;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::auth::AuthorizationResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::ntddk::IoGetCurrentProcess;

pub mod memory_services;
pub mod process_services;
pub mod security_services;
pub mod thread_services;

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
    let plugin = PluginTable::lookup(request.uuid);

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

pub fn handle_thread_services(
    guest: &mut dyn Guest,
    request: &HypervisorRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) {
    let result = match request.call.func() {
        ServiceFunction::OpenThread => open_thread_async(
            guest,
            OpenThreadRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::SuspendResumeThread => {
            suspend_resume_thread_async(
                guest,
                SuspendResumeThreadRequest::from_raw(request),
                plugin,
                async_info,
            )
        }
        ServiceFunction::KillThread => kill_thread_async(
            guest,
            KillThreadRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::GetThreadField => get_thread_field_async(
            guest,
            GetThreadFieldRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::CloseThread => {
            close_thread(guest, CloseThreadRequest::from_raw(request), plugin)
        }
        _ => unreachable!(),
    };

    write_response(guest, result);
}

pub fn handle_security_services(
    guest: &mut dyn Guest,
    request: &HypervisorRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) {
    if !plugin.perm_check(PluginPermissions::SECURITY_MANAGE) {
        write_response(
            guest,
            HypervisorResponse::not_allowed_perms(PluginPermissions::SECURITY_MANAGE),
        );
        return;
    }

    let result = match request.call.func() {
        ServiceFunction::OpenToken => open_token_async(
            guest,
            OpenTokenRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::GetTokenField => get_token_field_async(guest, GetTokenFieldRequest::from_raw(request), plugin, async_info),
        _ => unreachable!("forgot to implement this one"),
    };

    write_response(guest, result);
}

///
/// # Handle Memory Services
///
/// Dispatches the memory service request to [memory_services].
///
pub fn handle_memory_services(
    guest: &mut dyn Guest,
    request: &HypervisorRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) {
    if !request.call.is_async() {
        write_response(
            guest,
            HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
        );
        return;
    }

    let result = match request.call.func() {
        ServiceFunction::ProcessVMOperation => process_vm_operation_async(
            guest,
            RWProcessMemoryRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::ProtectProcessMemory => protect_vm_async(
            guest,
            ProtectProcessMemoryRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::AllocateMemory => allocate_mdl_async(
            guest,
            AllocateMemoryRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::MapMemory => map_mdl_async(
            guest,
            MapMemoryRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::FreeMemory => free_mdl_async(
            guest,
            FreeMemoryRequest::from_raw(request),
            plugin,
            async_info,
        ),
        _ => unreachable!("forgot to implement this one"),
    };

    write_response(guest, result)
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
        ServiceFunction::OpenProcess => open_process_async(
            guest,
            OpenProcessRequest::from_raw(request),
            plugin,
            async_info,
        ),
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
        ServiceFunction::GetProcessThreads => get_process_threads_async(
            guest,
            GetProcessThreadsRequest::from_raw(request),
            plugin,
            async_info,
        ),
        ServiceFunction::KillProcess => kill_process_async(
            guest,
            KillProcessRequest::from_raw(request),
            plugin,
            async_info,
        ),
        _ => unreachable!(),
    };

    write_response(guest, result);
}
