use crate::nt::process::NtProcess;
use crate::nt::*;
use crate::plugins::commands::process::*;
use crate::plugins::{Plugin, PluginTable};
use alloc::boxed::Box;
use core::ptr::copy_nonoverlapping;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::process::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::ObjectType;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use hxposed_core::services::types::process_fields::*;
use wdk_sys::ntddk::ObfDereferenceObject;

///
/// # Get Process Threads (Sync)
///
/// Gets number of threads for a process, with their ids.
///
/// ## Arguments
/// * `request` - Arguments for the request. See [`GetProcessThreadsAsyncCommand`].
///
/// ## Warning
/// - Caller must signal the request *after* calling this function.
///
/// ## Return
/// * [`HypervisorResponse::nt_error`] - An error occurred writing to the user buffer.
/// * [`HypervisorResponse::invalid_params`] - Invalid buffer.
/// * [`GetProcessThreadsResponse`] - Number of threads. Depending on if the buffer was provided, the thread ids are written to it.
pub(crate) fn get_process_threads_sync(
    request: &GetProcessThreadsAsyncCommand,
) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match plugin
        .object_table
        .get_open_process(request.command.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    let thread_numbers = process.get_threads();

    if !request.command.data.is_null() {
        match probe::probe_for_write(request.command.data, request.command.data_len, 1) {
            Ok(_) => {}
            Err(_) => return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
        }

        unsafe {
            copy_nonoverlapping::<u32>(
                thread_numbers.as_ptr(),
                request.command.data as _,
                request.command.data_len / 4, // calculate number of items to transfer.
            )
        }
    }

    GetProcessThreadsResponse {
        number_of_threads: thread_numbers.len() as _,
    }
    .into_raw()
}

///
/// # Get Process Threads
///
/// Queues command for [`GetProcessThreadsRequest`] on specified plugin.
///
/// ## Arguments
/// * `guest` - Currently unused.
/// * `request` - Identifies the target process and the exit code to use. See [`GetProcessThreadsRequest`].
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
pub(crate) fn get_process_threads_async(
    _guest: &mut dyn Guest,
    request: GetProcessThreadsRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(GetProcessThreadsAsyncCommand {
        command: request,
        async_info,
        uuid: plugin.uuid,
    }));

    EmptyResponse::with_service(ServiceFunction::GetProcessThreads)
}

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

    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(SetProcessFieldAsyncCommand {
        uuid: plugin.uuid,
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
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let mut process = match plugin
        .object_table
        .pop_open_process(request.command.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    let ret = match request.command.field {
        ProcessField::Protection => {
            if request.command.data_len != 1 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            match probe::probe_for_read(request.command.data, request.command.data_len, 1) {
                Ok(_) => {
                    let new_field = unsafe { *(request.command.data as *mut ProcessProtection) };

                    process.set_protection(new_field);

                    EmptyResponse::with_service(ServiceFunction::SetProcessField)
                }
                Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
            }
        }
        ProcessField::Signers => {
            if request.command.data_len != 2 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            match probe::probe_for_read(request.command.data, request.command.data_len, 1) {
                Ok(_) => {
                    let new_field =
                        unsafe { *(request.command.data as *mut ProcessSignatureLevel) };

                    process.set_signers(new_field);

                    EmptyResponse::with_service(ServiceFunction::SetProcessField)
                }
                Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
            }
        }
        ProcessField::MitigationFlags => {
            if request.command.data_len != 8 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            match probe::probe_for_read(request.command.data, request.command.data_len, 1) {
                Ok(_) => {
                    let mitigations = unsafe { *(request.command.data as *mut MitigationOptions) };

                    process.set_mitigations(mitigations);

                    EmptyResponse::with_service(ServiceFunction::SetProcessField)
                }
                Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
            }
        }
        ProcessField::Token => {
            if request.command.data_len != 8 {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            let token = match plugin
                .object_table
                .get_open_token(request.command.data as _)
            {
                Some(x) => x,
                None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
            };

            process.set_token(token.nt_token);

            EmptyResponse::with_service(ServiceFunction::SetProcessField)
        }
        _ => HypervisorResponse::not_found(),
    };

    // now, readd it.
    plugin.object_table.add_open_process(process);

    ret
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

    let obj = GetProcessFieldAsyncCommand {
        uuid: plugin.uuid,
        command: request,
        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::KillProcess)
        }
        false => match obj.command.field {
            ProcessField::NtPath | ProcessField::Token => {
                HypervisorResponse::invalid_params(ServiceParameter::IsAsync)
            }
            ProcessField::Protection | ProcessField::MitigationFlags | ProcessField::Signers => {
                get_process_field_sync(&obj)
            }
            ProcessField::Unknown => {
                HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction)
            }
        },
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
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match plugin
        .object_table
        .get_open_process(request.command.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.field {
        ProcessField::NtPath => {
            let field = unsafe { &mut *process.nt_path };

            if request.command.data_len == 0 {
                GetProcessFieldResponse::NtPath(field.Length)
            } else {
                match probe::probe_for_write(
                    request.command.data as _,
                    request.command.data_len as _,
                    1,
                ) {
                    Ok(_) => {
                        unsafe {
                            field.Buffer.copy_to_nonoverlapping(
                                request.command.data as *mut u16,
                                field.Length as usize / 2,
                            )
                        }
                        GetProcessFieldResponse::NtPath(field.Length)
                    }
                    Err(_) => {
                        return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
                    }
                }
            }
        }
        ProcessField::Protection => {
            GetProcessFieldResponse::Protection(process.get_protection().into_bits() as _)
        }
        ProcessField::Signers => {
            GetProcessFieldResponse::Signers(unsafe { process.get_signers().into_bits() as _ })
        }
        ProcessField::MitigationFlags => GetProcessFieldResponse::Mitigation(unsafe {
            process.get_mitigations().into_bits() as _
        }),
        ProcessField::Token => {
            if !plugin.perm_check(PluginPermissions::PROCESS_SECURITY) {
                return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_SECURITY);
            }
            GetProcessFieldResponse::Token(process.get_token() as _)
        }
        _ => return HypervisorResponse::not_found(),
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

    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(KillProcessAsyncCommand {
        uuid: plugin.uuid,
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
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match plugin
        .object_table
        .pop_open_process(request.command.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match process.kill(request.command.exit_code as _) {
        Ok(_) => EmptyResponse::with_service(ServiceFunction::KillProcess),
        Err(err) => HypervisorResponse::nt_error(err as _),
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
    match plugin.object_table.pop_open_process(request.process as _) {
        None => HypervisorResponse::not_found(),
        Some(process) => {
            unsafe {
                ObfDereferenceObject(process.uid as _);
            }
            EmptyResponse::with_service(ServiceFunction::CloseProcess)
        }
    }
}

pub(crate) fn open_process_async(
    _guest: &mut dyn Guest,
    request: OpenProcessRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::PROCESS_EXECUTIVE);
    }

    let obj = OpenProcessAsyncCommand {
        command: request,
        async_info,
        uuid: plugin.uuid,
    };

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::OpenProcess)
        }
        false => match obj.command.open_type {
            ObjectOpenType::Handle => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
            ObjectOpenType::Hypervisor => open_process_sync(&obj),
        },
    }
}

///
/// # Open Process (Sync)
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
pub(crate) fn open_process_sync(request: &OpenProcessAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let process = match NtProcess::from_id(request.command.process_id) {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    match request.command.open_type {
        ObjectOpenType::Handle => OpenObjectResponse {
            object: ObjectType::Handle(match process.open_handle() {
                Ok(handle) => handle.get_forget() as _,
                Err(x) => return HypervisorResponse::nt_error(x as _),
            }),
        }
        .into_raw(),
        ObjectOpenType::Hypervisor => {
            let uid = process.uid;
            plugin.object_table.add_open_process(process);

            OpenObjectResponse {
                object: ObjectType::Process(uid) as _,
            }
            .into_raw()
        }
    }
}
