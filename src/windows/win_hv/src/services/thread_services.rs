use crate::nt::blanket::OpenHandle;
use crate::plugins::commands::thread::*;
use crate::plugins::{Plugin, PluginTable};
use crate::win::{ZwResumeThread, ZwSuspendThread};
use alloc::boxed::Box;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::thread::SuspendThreadResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::ntddk::PsLookupThreadByThreadId;
use wdk_sys::{PETHREAD, STATUS_SUCCESS, ULONG};

pub(crate) fn suspend_resume_thread_sync(
    request: &SuspendResumeThreadAsyncCommand,
) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let thread = match plugin
        .object_table
        .get_open_thread(Some(request.command.id), None)
    {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    let handle = match thread.get_handle() {
        Ok(handle) => handle,
        Err(x) => return HypervisorResponse::nt_error(x as _),
    };

    let mut previous_count = ULONG::default();

    match request.command.operation {
        SuspendResumeThreadOperation::Suspend => {
            match unsafe { ZwSuspendThread(handle.get_danger(), &mut previous_count) } {
                STATUS_SUCCESS => SuspendThreadResponse { previous_count }.into_raw(),
                err => HypervisorResponse::nt_error(err as u32),
            }
        }
        SuspendResumeThreadOperation::Resume => {
            match unsafe { ZwResumeThread(handle.get_danger(), &mut previous_count) } {
                STATUS_SUCCESS => SuspendThreadResponse { previous_count }.into_raw(),
                err => HypervisorResponse::nt_error(err as u32),
            }
        }
        SuspendResumeThreadOperation::Freeze => {
            HypervisorResponse::invalid_params(ServiceParameter::Function)
        }
    }
}

pub(crate) fn suspend_resume_thread_async(
    _guest: &mut dyn Guest,
    request: SuspendResumeThreadRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::THREAD_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_EXECUTIVE);
    }

    plugin.queue_command(Box::new(SuspendResumeThreadAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::SuspendResumeThread)
}

///
/// # Open Thread (sync)
///
/// References `_ETHREAD` to plugin's virtual object table.
///
/// ## Return
/// * [`HypervisorResponse::not_found_what`] - Not found.
/// * [`HypervisorResponse::nt_error`] - NT side error.
/// * [`OpenObjectResponse`] - Object's address (or handle)
pub(crate) fn open_thread_sync(request: &OpenThreadAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let mut thread = PETHREAD::default();

    match unsafe { PsLookupThreadByThreadId(request.command.tid as _, &mut thread) } {
        STATUS_SUCCESS => {}
        err => return HypervisorResponse::nt_error(err as _),
    }

    match request.command.open_type {
        ObjectOpenType::Handle => OpenObjectResponse {
            addr: match thread.get_handle() {
                Ok(handle) => handle.get_forget() as _,
                Err(x) => return HypervisorResponse::nt_error(x as _),
            },
        }
        .into_raw(),
        ObjectOpenType::Hypervisor => {
            plugin.object_table.open_threads.push(thread);
            OpenObjectResponse { addr: thread as _ }.into_raw()
        }
    }
}

///
/// # Open Thread
///
/// Opens a thread for plugin. Note that success means the work is queued, not necessarily completed for in case of [`ObjectOpenType::Handle`]
///
/// ## Return
/// * [`EmptyResponse`] - OK.
/// * [`HypervisorResponse::not_found`] - Thread was not found.
/// * [`HypervisorResponse::not_allowed_perms`] - Plugin lacks required permissions.
/// * [`HypervisorResponse::invalid_params`] - Plugin provided invalid call type.
pub(crate) fn open_thread_async(
    _guest: &mut dyn Guest,
    request: OpenThreadRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::THREAD_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_EXECUTIVE);
    }

    let obj = OpenThreadAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    };

    match obj.command.open_type {
        ObjectOpenType::Handle => {
            if !obj.async_info.is_present() {
                return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
            }

            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::OpenThread)
        }
        ObjectOpenType::Hypervisor => open_thread_sync(&obj),
    }
}

///
/// # Close Thread
///
/// Closes the thread, consumes the object from plugin's virtual object table.
///
/// ## Return
/// * [`EmptyResponse`] - OK.
/// * [`HypervisorResponse::not_found`] - Thread was not found.
pub(crate) fn close_thread(
    _guest: &mut dyn Guest,
    request: CloseThreadRequest,
    plugin: &'static mut Plugin,
) -> HypervisorResponse {
    match plugin.object_table.pop_open_thread(request.addr as _) {
        None => HypervisorResponse::not_found(),
        Some(_) => EmptyResponse::with_service(ServiceFunction::CloseThread),
    }
}
