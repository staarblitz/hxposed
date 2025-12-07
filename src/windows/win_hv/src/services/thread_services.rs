use crate::plugins::commands::thread::OpenThreadAsyncCommand;
use crate::plugins::{Plugin, PluginTable};
use alloc::boxed::Box;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{CloseProcessRequest, ObjectOpenType};
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::ntddk::{ObOpenObjectByPointer, PsLookupThreadByThreadId};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{PsThreadType, HANDLE, PETHREAD, STATUS_SUCCESS, THREAD_ALL_ACCESS};

pub(crate) fn suspend_thread_async(
    _guest: &mut dyn Guest,
    request: SuspendThreadRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    EmptyResponse::with_service(ServiceFunction::SuspendThread)
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
        ObjectOpenType::Handle => {
            let mut handle = HANDLE::default();

            match unsafe {
                ObOpenObjectByPointer(
                    thread as _,
                    0,
                    Default::default(),
                    THREAD_ALL_ACCESS,
                    *PsThreadType,
                    KernelMode as _,
                    &mut handle,
                )
            } {
                STATUS_SUCCESS => OpenObjectResponse { addr: handle as _ }.into_raw(),
                err => HypervisorResponse::nt_error(err as _),
            }
        }
        ObjectOpenType::Hypervisor => {
            plugin.object_table.open_threads.push(thread);
            OpenObjectResponse { addr: thread as _ }.into_raw()
        },
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