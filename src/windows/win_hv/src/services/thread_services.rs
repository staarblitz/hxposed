use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::thread::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub(crate) fn kill_thread_sync() -> HypervisorResponse {
    EmptyResponse::with_service(ServiceFunction::KillThread)
    /*let thread = match ObjectTracker::get_open_thread(request.command.thread as _) {
        Some(thread) => thread.take(),
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match thread.kill(request.command.exit_code as _) {
        Ok(_) => EmptyResponse::with_service(ServiceFunction::KillThread),
        Err(err) => HypervisorResponse::nt_error(err as _),
    }*/
}

pub(crate) fn get_thread_field_sync(request: GetThreadFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    let mut thread = match tracker.get_open_thread(request.thread) {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.field {
        ThreadField::ActiveImpersonationInfo(_) => {
            GetThreadFieldResponse::ActiveImpersonationInfo(thread.get_impersonation_info())
        }
        ThreadField::AdjustedClientToken(_) => {
            GetThreadFieldResponse::AdjustedClientToken(thread.get_adjusted_client_token() as _)
        }
        _ => return HypervisorResponse::not_found(),
    }
    .into_raw()
}

pub(crate) fn set_thread_field_sync(request: SetThreadFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let thread = match process
        .get_object_tracker_unchecked()
        .get_open_thread(request.thread)
    {
        Some(thread) => thread,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.field {
        ThreadField::AdjustedClientToken(token) => {
            let token = match process.get_object_tracker_unchecked().get_open_token(token) {
                Some(x) => x,
                None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
            };

            thread.set_adjusted_client_token(token.nt_token);

            EmptyResponse::with_service(ServiceFunction::SetThreadField)
        }
        _ => HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction),
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
pub(crate) fn open_thread_sync(request: OpenThreadRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let thread = match NtThread::from_id(request.tid) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.open_type {
        ObjectOpenType::Handle => OpenObjectResponse {
            object: ObjectType::Handle(thread.open_handle().get_forget() as _),
        }
        .into_raw(),
        ObjectOpenType::Hypervisor => {
            let rep = OpenObjectResponse {
                object: ObjectType::Token(thread.nt_thread as _),
            }
            .into_raw();
            process
                .get_object_tracker_unchecked()
                .add_open_thread(thread);
            rep
        }
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
pub(crate) fn close_thread_sync(request: CloseThreadRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    match process
        .get_object_tracker_unchecked()
        .pop_open_thread(request.thread)
    {
        None => HypervisorResponse::not_found(),
        Some(x) => {
            drop(x);
            EmptyResponse::with_service(ServiceFunction::CloseThread)
        }
    }
}
