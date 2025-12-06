use crate::nt::{EProcessField, get_eprocess_field};
use crate::plugins::commands::process::*;
use crate::plugins::plugin::Plugin;
use crate::win::PsTerminateProcess;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicPtr, Ordering};
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::process::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use hxposed_core::services::types::process_fields::{ProcessProtection, ProcessSignatureLevels};
use wdk_sys::ntddk::{ProbeForRead, ProbeForWrite, PsLookupProcessByProcessId};
use wdk_sys::{_UNICODE_STRING, PEPROCESS, STATUS_SUCCESS};

///
/// # Set Process Field
///
/// Queues command for [`SetProcessFieldRequest`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`SetProcessFieldRequest`].
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
pub(crate) fn set_process_field_async(
    _guest: &mut dyn Guest,
    request: SetProcessFieldRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    plugin.queue_command(Box::new(SetProcessFieldAsyncCommand {
        process,
        plugin_process: plugin.process.load(Ordering::Relaxed),
        command: request,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::SetProcessField)
}

///
/// # Set Process Field (Sync)
///
/// Sets a field from executive process object.
///
/// ## Arguments
/// * `request` - Arguments for the request. See [`SetProcessFieldAsyncCommand`].
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::nt_error`] - An error occurred writing to the user buffer.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`HypervisorResponse::invalid_params`] - Invalid buffer.
/// * [`GetProcessFieldResponse::NtPath`] - Number of bytes for the name. Also, depending on if the caller allocated the buffer, name is written to buffer.
pub(crate) fn set_process_field_sync(request: &SetProcessFieldAsyncCommand) -> HypervisorResponse {
    match request.command.field {
        ProcessField::Protection => {
            if request.command.data_len != 1 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            let field = unsafe {
                get_eprocess_field::<ProcessProtection>(EProcessField::Protection, request.process)
            };

            match microseh::try_seh(|| unsafe {
                ProbeForRead(request.command.data as _, request.command.data_len as _, 1)
            }) {
                Ok(_) => {
                    let new_field = unsafe { *(request.command.data as *mut ProcessProtection) };

                    unsafe { field.write(new_field) };

                    EmptyResponse::with_service(ServiceFunction::SetProcessField)
                }
                Err(x) => HypervisorResponse::nt_error(x.code() as _),
            }
        }
        ProcessField::Signers => {
            if request.command.data_len != 2 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            let field = unsafe {
                get_eprocess_field::<ProcessSignatureLevels>(
                    EProcessField::SignatureLevels,
                    request.process,
                )
            };

            match microseh::try_seh(|| unsafe {
                ProbeForRead(request.command.data as _, request.command.data_len as _, 2)
            }) {
                Ok(_) => {
                    let new_field =
                        unsafe { *(request.command.data as *mut ProcessSignatureLevels) };
                    unsafe { field.write(new_field) };

                    EmptyResponse::with_service(ServiceFunction::SetProcessField)
                }
                Err(x) => HypervisorResponse::nt_error(x.code() as _),
            }
        }
        _ => HypervisorResponse::not_found(),
    }
}

///
/// # Get Process Field
///
/// Queues command for [`GetProcessFieldRequest`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`GetProcessFieldRequest`].
/// * `plugin` - The plugin requesting the operation. See [`Plugin`].
/// * `async_handle` - Handle object plugin created.
///
/// ## Warning
/// - This function only enqueues the request; success does **not** imply the process was actually terminated. (See the code for more information)
///
/// ## Return
/// * [`HypervisorResponse::not_found`] - The specified process does not exist.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`HypervisorResponse::ok`] - The request was successfully enqueued.
pub(crate) fn get_process_field_async(
    _guest: &mut dyn Guest,
    request: GetProcessFieldRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    match request.field {
        ProcessField::NtPath => {
            if !async_info.is_present() {
                return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
            }

            plugin.queue_command(Box::new(GetProcessFieldAsyncCommand {
                process,
                plugin_process: plugin.process.load(Ordering::Relaxed),
                command: request,
                async_info,
            }));
            EmptyResponse::with_service(ServiceFunction::KillProcess)
        }
        // directly call the sync counterpart.
        ProcessField::Protection | ProcessField::Signers => {
            get_process_field_sync(&GetProcessFieldAsyncCommand {
                process,
                plugin_process: plugin.process.load(Ordering::Relaxed),
                command: request,
                async_info,
            })
        }
        _ => HypervisorResponse::not_found(),
    }
}

///
/// # Get Process Field (Sync)
///
/// Gets a field from executive process object.
///
/// ## Arguments
/// * `request` - Arguments for the request. See [`GetProcessFieldAsyncCommand`].
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::nt_error`] - An error occurred writing to the user buffer.
/// * [`HypervisorResponse::not_allowed_perms`] - The plugin lacks the required permissions.
/// * [`GetProcessFieldResponse::NtPath`] - Number of bytes for the name. Also, depending on if the caller allocated the buffer, name is written to buffer.
///
pub(crate) fn get_process_field_sync(request: &GetProcessFieldAsyncCommand) -> HypervisorResponse {
    match request.command.field {
        ProcessField::NtPath => {
            let field = unsafe {
                &mut **get_eprocess_field::<*mut _UNICODE_STRING>(
                    EProcessField::SeAuditProcessCreationInfo,
                    request.process,
                )
            };

            if request.command.data_len == 0 {
                GetProcessFieldResponse::NtPath(field.Length)
            } else {
                match microseh::try_seh(|| unsafe {
                    ProbeForWrite(request.command.data as _, request.command.data_len as _, 1)
                }) {
                    Ok(_) => {
                        unsafe {
                            field.Buffer.copy_to_nonoverlapping(
                                request.command.data as *mut u16,
                                field.Length as usize / 2,
                            )
                        }
                        GetProcessFieldResponse::NtPath(field.Length)
                    }
                    Err(x) => return HypervisorResponse::nt_error(x.code() as _),
                }
            }
        }
        ProcessField::Protection => GetProcessFieldResponse::Protection(
            unsafe {
                *get_eprocess_field::<ProcessProtection>(EProcessField::Protection, request.process)
            }
            .into_bits() as _,
        ),
        ProcessField::Signers => GetProcessFieldResponse::Signers(unsafe {
            *get_eprocess_field::<u16>(EProcessField::SignatureLevels, request.process)
        }),
        _ => GetProcessFieldResponse::Unknown,
    }
    .into_raw()
}

///
/// # Kill Process
///
/// Queues command for [`KillProcessRequest`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`KillProcessRequest`].
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
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    let process = match plugin.get_open_process(Some(request.id), None) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    plugin.queue_command(Box::new(KillProcessAsyncCommand {
        process,
        plugin_process: plugin.process.load(Ordering::Relaxed),
        command: request,
        async_info,
    }));

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
pub(crate) fn kill_process_sync(request: &KillProcessAsyncCommand) -> HypervisorResponse {
    match unsafe { PsTerminateProcess(request.process, request.command.exit_code as _) } {
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
