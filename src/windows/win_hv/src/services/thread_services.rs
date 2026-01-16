use crate::nt::thread::NtThread;
use crate::objects::ObjectTracker;
use crate::services::commands::thread::*;
use crate::win::{ZwResumeThread, ZwSuspendThread};
use alloc::boxed::Box;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::thread::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use wdk_sys::{STATUS_SUCCESS, ULONG};

pub(crate) fn kill_thread_sync(request: &KillThreadAsyncCommand) -> HypervisorResponse {
    let thread = match ObjectTracker::get_open_thread(request.command.thread as _) {
        Some(thread) => thread.take(),
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match thread.kill(request.command.exit_code as _) {
        Ok(_) => EmptyResponse::with_service(ServiceFunction::KillThread),
        Err(err) => HypervisorResponse::nt_error(err as _),
    }
}

pub(crate) fn kill_thread_async(
    request: KillThreadRequest,

    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    ObjectTracker::queue_command(Box::new(KillThreadAsyncCommand {
        command: request,

        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::KillThread)
}

pub(crate) fn suspend_resume_thread_sync(
    request: &SuspendResumeThreadAsyncCommand,
) -> HypervisorResponse {
    let thread = match ObjectTracker::get_open_thread(request.command.thread as _) {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    let handle = match thread.open_handle() {
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
    request: SuspendResumeThreadRequest,

    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    ObjectTracker::queue_command(Box::new(SuspendResumeThreadAsyncCommand {
        command: request,

        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::SuspendResumeThread)
}

pub(crate) fn get_thread_field_sync(request: &GetThreadFieldAsyncCommand) -> HypervisorResponse {
    let thread = match ObjectTracker::get_open_thread(request.command.thread as _) {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.field {
        ThreadField::ActiveImpersonationInfo => {
            GetThreadFieldResponse::ActiveImpersonationInfo(thread.get_impersonation_info())
        }
        ThreadField::AdjustedClientToken => {
            GetThreadFieldResponse::AdjustedClientToken(thread.get_adjusted_client_token() as _)
        }
        _ => return HypervisorResponse::not_found(),
    }
    .into_raw()
}

pub(crate) fn set_thread_field_sync(request: &SetThreadFieldAsyncCommand) -> HypervisorResponse {
    let mut thread = match ObjectTracker::get_open_thread(request.command.thread as _) {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.field {
        ThreadField::AdjustedClientToken => {
            let token = match ObjectTracker::get_open_token(request.command.data as _) {
                Some(x) => x,
                None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
            };

            thread.set_adjusted_client_token(token.nt_token);

            EmptyResponse::with_service(ServiceFunction::SetThreadField)
        }
        _ => HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction),
    }
}
pub(crate) fn set_thread_field_async(
    request: SetThreadFieldRequest,

    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    let obj = SetThreadFieldAsyncCommand {
        command: request,

        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            ObjectTracker::queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::SetThreadField)
        }
        false => match obj.command.field {
            _ => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
        },
    }
}

pub(crate) fn get_thread_field_async(
    request: GetThreadFieldRequest,

    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    let obj = GetThreadFieldAsyncCommand {
        command: request,

        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            ObjectTracker::queue_command(Box::new(obj));
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
    let thread = match NtThread::from_id(request.command.tid) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.command.open_type {
        ObjectOpenType::Handle => OpenObjectResponse {
            object: ObjectType::Handle(match thread.open_handle() {
                Ok(handle) => handle.get_forget() as _,
                Err(x) => return HypervisorResponse::nt_error(x as _),
            }),
        }
        .into_raw(),
        ObjectOpenType::Hypervisor => {
            let rep = OpenObjectResponse {
                object: ObjectType::Token(thread.nt_thread as _),
            }
            .into_raw();
            ObjectTracker::add_open_thread(thread);
            rep
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
    request: OpenThreadRequest,

    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    let obj = OpenThreadAsyncCommand {
        command: request,

        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            ObjectTracker::queue_command(Box::new(obj));
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
pub(crate) fn close_thread(request: CloseThreadRequest) -> HypervisorResponse {
    match ObjectTracker::get_open_thread(request.thread as _) {
        None => HypervisorResponse::not_found(),
        Some(x) => {
            x.take();
            EmptyResponse::with_service(ServiceFunction::CloseThread)
        }
    }
}
