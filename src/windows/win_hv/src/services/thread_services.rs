use crate::nt::blanket::OpenHandle;
use crate::nt::{EThreadField, get_ethread_field};
use crate::plugins::commands::thread::*;
use crate::plugins::{Plugin, PluginTable};
use crate::win::{PspTerminateThread, ZwResumeThread, ZwSuspendThread};
use alloc::boxed::Box;
use bit_field::BitField;
use core::arch::asm;
use core::sync::atomic::Ordering;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::thread::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::ntddk::{
    PsLookupThreadByThreadId, PsReferenceImpersonationToken, PsReferencePrimaryToken,
};
use wdk_sys::{
    BOOLEAN, PBOOLEAN, PETHREAD, PSECURITY_IMPERSONATION_LEVEL, SECURITY_IMPERSONATION_LEVEL,
    STATUS_SUCCESS, ULONG,
};

pub(crate) fn kill_thread_sync(request: &KillThreadAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let thread = match plugin
        .object_table
        .get_open_thread(request.command.addr as _)
    {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    unsafe {
        asm!("mov r15, r14", in("r14") crate::win::NT_PS_TERMINATE_THREAD.load(Ordering::Relaxed));
    }
    match unsafe { PspTerminateThread(thread, request.command.exit_code as _, 1) } {
        STATUS_SUCCESS => EmptyResponse::with_service(ServiceFunction::KillThread),
        err => HypervisorResponse::nt_error(err as _),
    }
}

pub(crate) fn kill_thread_async(
    _guest: &mut dyn Guest,
    request: KillThreadRequest,
    plugin: &'static mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::THREAD_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_EXECUTIVE);
    }

    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(KillThreadAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::KillThread)
}

pub(crate) fn suspend_resume_thread_sync(
    request: &SuspendResumeThreadAsyncCommand,
) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let thread = match plugin
        .object_table
        .get_open_thread(request.command.addr as _)
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

    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(SuspendResumeThreadAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::SuspendResumeThread)
}

pub(crate) fn get_thread_field_sync(request: &GetThreadFieldAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let thread = match plugin
        .object_table
        .get_open_thread(request.command.addr as _)
    {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.field {
        ThreadField::ActiveImpersonationInfo => {
            if !plugin.perm_check(PluginPermissions::THREAD_SECURITY) {
                return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_SECURITY);
            }

            let field =
                unsafe { *get_ethread_field::<u32>(EThreadField::CrossThreadFlags, thread) }
                    .get_bit(3);

            GetThreadFieldResponse::ActiveImpersonationInfo(field)
        }
        ThreadField::AdjustedClientToken => {
            if !plugin.perm_check(PluginPermissions::THREAD_SECURITY) {
                return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_SECURITY);
            }

            let mut copy_on_open = BOOLEAN::default();
            let mut effective_only = BOOLEAN::default();
            let mut impersonation_level = SECURITY_IMPERSONATION_LEVEL::default();

            let field = unsafe {
                PsReferenceImpersonationToken(
                    thread,
                    &mut copy_on_open,
                    &mut effective_only,
                    &mut impersonation_level,
                )
            };

            log::warn!("PsReferenceImpersonationToken: copy_on_open: {:?}, effective_only: {:?}, impersonation_level: {:?}", copy_on_open, effective_only, impersonation_level);

            GetThreadFieldResponse::AdjustedClientToken(field as _)
        }
        _ => GetThreadFieldResponse::Unknown,
    }
    .into_raw()
}

pub(crate) fn set_thread_field_sync(request: &SetThreadFieldAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let thread = match plugin
        .object_table
        .get_open_thread(request.command.addr as _)
    {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.field {
        ThreadField::AdjustedClientToken => {
            if !plugin.perm_check(PluginPermissions::THREAD_SECURITY) {
                return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_SECURITY);
            }

            let token = match plugin
                .object_table
                .get_open_token(request.command.data as _)
            {
                Some(x) => x,
                None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
            };

            let field =
                unsafe { get_ethread_field::<*mut u64>(EThreadField::AdjustedClientToken, thread) };

            unsafe { field.write(token as _) };

            EmptyResponse::with_service(ServiceFunction::SetThreadField)
        }
        _ => HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction),
    }
}

pub(crate) fn set_thread_field_async(
    _guest: &mut dyn Guest,
    request: SetThreadFieldRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::THREAD_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_EXECUTIVE);
    }

    let obj = SetThreadFieldAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::SetThreadField)
        }
        false => match obj.command.field {
            _ => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
        },
    }
}

pub(crate) fn get_thread_field_async(
    _guest: &mut dyn Guest,
    request: GetThreadFieldRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::THREAD_EXECUTIVE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::THREAD_EXECUTIVE);
    }

    let obj = GetThreadFieldAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::SuspendResumeThread)
        }
        false => match obj.command.field {
            ThreadField::ActiveImpersonationInfo => get_thread_field_sync(&obj),
            _ => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
        },
    }
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

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::OpenThread)
        }
        false => match obj.command.open_type {
            ObjectOpenType::Handle => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
            ObjectOpenType::Hypervisor => open_thread_sync(&obj),
        },
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
