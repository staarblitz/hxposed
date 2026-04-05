use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::thread::*;
use hxposed_core::hxposed::responses::{HxResponse, OpenObjectResponse, SyscallResponse};
use hxposed_core::hxposed::ObjectType;
use crate::utils::logger::{HxLogger, LogEvent, LogType};

pub(crate) fn get_thread_field_sync(request: GetThreadFieldRequest) -> HxResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    let thread = match tracker.get_open_thread(request.thread) {
        Some(thread) => thread,
        None => return HxResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.field {
        ThreadField::Unknown => return HxResponse::invalid_params(0),
        ThreadField::ActiveImpersonationInfo(_) => {
            GetThreadFieldResponse::ActiveImpersonationInfo(thread.get_impersonation_info())
        }
        ThreadField::AdjustedClientToken(_) => {
            GetThreadFieldResponse::AdjustedClientToken(thread.get_adjusted_client_token() as _)
        }
    }
    .into_raw()
}

pub(crate) fn set_thread_field_sync(request: SetThreadFieldRequest) -> HxResponse {
    let process = NtProcess::current();
    let thread = match process
        .get_object_tracker_unchecked()
        .get_open_thread(request.thread)
    {
        Some(thread) => thread,
        None => return HxResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.field {
        ThreadField::AdjustedClientToken(token) => {
            let token = match process.get_object_tracker_unchecked().get_open_token(token) {
                Some(x) => x,
                None => return HxResponse::not_found_what(NotFoundReason::Token),
            };

            thread.set_adjusted_client_token(token.nt_token);

            EmptyResponse::default()
        }
        _ => HxResponse::invalid_params(0),
    }
}

///
/// # Open Thread (sync)
///
/// References `_ETHREAD` to plugin's virtual object table.
///
/// ## Return
/// * [`HxResponse::not_found_what`] - Not found.
/// * [`HxResponse::nt_error`] - NT side error.
/// * [`OpenObjectResponse`] - Object's address (or handle)
pub(crate) fn open_thread_sync(request: OpenThreadRequest) -> HxResponse {
    let process = NtProcess::current();

    HxLogger::serial_log(LogType::Trace, LogEvent::QueryObject(request.tid, process.nt_process as _ ));

    let thread = match NtThread::from_id(request.tid) {
        Some(x) => x,
        None => return HxResponse::not_found_what(NotFoundReason::Thread),
    };

    HxLogger::serial_log(LogType::Trace, LogEvent::TrackObject(thread.nt_thread as _, process.nt_process as _ ));
    OpenObjectResponse {
        object: ObjectType::Thread(process
            .get_object_tracker_unchecked()
            .add_open_thread(thread)),
    }.into_raw()
}
///
/// # Close Thread
///
/// Closes the thread, consumes the object from plugin's virtual object table.
///
/// ## Return
/// * [`EmptyResponse`] - OK.
/// * [`HxResponse::not_found`] - Thread was not found.
pub(crate) fn close_thread_sync(request: CloseThreadRequest) -> HxResponse {
    let process = NtProcess::current();
    match process
        .get_object_tracker_unchecked()
        .pop_open_thread(request.thread)
    {
        None => HxResponse::not_found_what(NotFoundReason::Thread),
        Some(x) => {
            HxLogger::serial_log(LogType::Trace, LogEvent::DetrackObject(x.nt_thread as _, process.nt_process as _ ));
            drop(x);
            EmptyResponse::default()
        }
    }
}
