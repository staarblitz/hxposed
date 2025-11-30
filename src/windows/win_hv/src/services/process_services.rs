use crate::plugins::async_command::KillProcessAsyncCommand;
use crate::plugins::plugin::Plugin;
use crate::win::PsTerminateProcess;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicPtr, Ordering};
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{
    CloseProcessRequest, KillProcessRequest, OpenProcessRequest,
};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::process::OpenProcessResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::AsyncInfo;
use wdk_sys::ntddk::PsLookupProcessByProcessId;
use wdk_sys::{HANDLE, PEPROCESS, STATUS_SUCCESS};

///
/// # Kill Process
///
/// Queues command for [KillProcessRequest] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use.See [`KillProcessRequest`].
/// * `plugin` - The plugin requesting the operation. See [`Plugin`].
/// * `async_handle` - Handle object plugin created.
///
/// ## Warning
/// - This function only enqueues the request; success does **not** imply the process was actually terminated.
///
/// ## Return
/// * [`HypervisorResponse::not_found`] - The specified process does not exist.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`HypervisorResponse::ok`] - The request was successfully enqueued.
///
/// TODO: Move the existence check to worker thread maybe?
pub(crate) fn kill_process_async(
    _guest: &mut dyn Guest,
    request: KillProcessRequest,
    plugin: &'static mut Plugin,
    async_info: &AsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found(),
    };

    plugin.queue_command(Box::new(KillProcessAsyncCommand::new(
        plugin.process.load(Ordering::Relaxed),
        request.exit_code,
        process,
        async_info,
    )));

    EmptyResponse::with_service(ServiceFunction::KillProcess)
}

///
/// # Kill Process (Sync)
///
/// Does actual work of killing the specified process.
///
/// ## Arguments
/// * `request` - Pointer to [`KillProcessAsyncCommand`]
/// * `plugin` - [`Plugin`] that asked for the service.
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::ok`] - The process was killed.
/// * [`HypervisorResponse::nt_error`] - [`PsTerminateProcess`] returned an NTSTATUS indicating failure.
pub(crate) fn kill_process_sync(
    request: &KillProcessAsyncCommand,
    plugin: &Plugin,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    match unsafe { PsTerminateProcess(request.process, request.exit_code as _) } {
        STATUS_SUCCESS => EmptyResponse::with_service(ServiceFunction::KillProcess),
        err => HypervisorResponse::nt_error(err as _),
    }
}

///
/// # Close Process
///
/// Closes a process in virtual handle table of plugin. For more information, visit "How Plugins Work" in Wiki.
///
/// ## Arguments
/// * `guest` - Unused.
/// * `request` - [`CloseProcessRequest`].
/// * `plugin` - [`Plugin`]
///
/// ## Return
/// * [`HypervisorResponse::ok`] - Process was closed.
/// * [`HypervisorResponse::not_allowed`] - Something went very wrong.
pub(crate) fn close_process(
    _guest: &mut dyn Guest,
    request: CloseProcessRequest,
    plugin: &'static mut Plugin,
) -> HypervisorResponse {
    if let Some((index, _)) = plugin
        .open_processes
        .iter()
        .enumerate()
        .find(|(_, x)| x.load(Ordering::Relaxed).addr() as u64 == request.addr)
    {
        plugin.open_processes.remove(index);
        EmptyResponse::with_service(ServiceFunction::CloseProcess)
    } else {
        // this is weird. a plugin should never attempt to close a process it has never opened in the first place.
        // abuse detected. blacklist the plugin (soon)
        HypervisorResponse::not_allowed(NotAllowedReason::Unknown)
    }
}

///
/// # Open Process
///
/// Opens a process in virtual handle table of plugin. For more information, visit "How Plugins Work" in Wiki.
///
/// ## Arguments
/// * `guest` - Unused.
/// * `request` - [`CloseProcessRequest`].
/// * `plugin` - [`Plugin`]
///
/// ## Return
/// * [`HypervisorResponse::ok`] - Process was opened.
/// * [`HypervisorResponse::not_allowed_perms`] - Plugin lacks required permissions
/// * [`HypervisorResponse::nt_error`] - [`PsLookupProcessByProcessId`] returned an NTSTATUS indicating failure.
pub(crate) fn open_process(
    _guest: &mut dyn Guest,
    request: OpenProcessRequest,
    plugin: &'static mut Plugin,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }
    let mut process = PEPROCESS::default();

    match unsafe { PsLookupProcessByProcessId(request.process_id as _, &mut process) } {
        STATUS_SUCCESS => {}
        err => return HypervisorResponse::nt_error(err as _),
    }

    plugin.open_processes.push(AtomicPtr::new(process));

    OpenProcessResponse { addr: process as _ }.into_raw()
}
