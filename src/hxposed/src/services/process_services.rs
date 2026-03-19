use crate::nt::process::NtProcess;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::process::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

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
pub(crate) fn set_process_field_sync(request: SetProcessFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let process = match process
        .get_object_tracker_unchecked()
        .get_open_process(request.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    match request.field {
        ProcessField::Protection(protection) => {
            process.set_protection(protection);
            EmptyResponse::default()
        }
        ProcessField::Signers(signers) => {
            process.set_signers(signers);
            EmptyResponse::default()
        }
        ProcessField::MitigationFlags(flags) => {
            process.set_mitigations(flags);
            EmptyResponse::default()
        }
        ProcessField::DirectoryTableBase(base) => {
            process.set_directory_table_base(base);
            EmptyResponse::default()
        }
        ProcessField::UserDirectoryTableBase(base) => {
            process.set_user_directory_table_base(base);
            EmptyResponse::default()
        }
        ProcessField::Token(token) => {
            let token = match process.get_object_tracker_unchecked().get_open_token(token) {
                Some(x) => x,
                None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
            };

            process.set_token(token.nt_token);

            EmptyResponse::default()
        }
        _ => HypervisorResponse::not_found_what(NotFoundReason::Field),
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
pub(crate) fn get_process_field_sync(request: GetProcessFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let state = process.get_hx_async_state_unchecked();
    let process = match process
        .get_object_tracker_unchecked()
        .get_open_process(request.process as _)
    {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Thread),
    };

    let field = match request.field {
        ProcessField::NtPath(_) => {
            let field = process.get_nt_path();
            let raw_string = field.get_raw_bytes();
            let offset = state.write_result(raw_string);
            ProcessField::NtPath(offset)
        }
        ProcessField::Protection(_) => ProcessField::Protection(process.get_protection()),
        ProcessField::Signers(_) => ProcessField::Signers(process.get_signers()),
        ProcessField::MitigationFlags(_) => {
            ProcessField::MitigationFlags(process.get_mitigations())
        }
        ProcessField::Token(_) => ProcessField::Token(process.get_token() as _),
        ProcessField::Threads(_) => {
            let thread_numbers = process.get_threads();
            let offset = state.write_result(thread_numbers.as_slice());
            ProcessField::Threads(offset)
        }
        ProcessField::DirectoryTableBase(_) => {
            ProcessField::DirectoryTableBase(process.get_directory_table_base().into())
        }
        ProcessField::UserDirectoryTableBase(_) => {
            ProcessField::DirectoryTableBase(process.get_user_directory_table_base().into())
        },
        ProcessField::Unknown => ProcessField::Unknown,
    };

    GetProcessFieldResponse { field }.into_raw()
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
pub(crate) fn close_process(request: CloseProcessRequest) -> HypervisorResponse {
    match NtProcess::current()
        .get_object_tracker_unchecked()
        .pop_open_process(request.process as _)
    {
        None => HypervisorResponse::not_found_what(NotFoundReason::Process),
        Some(process) => {
            drop(process);
            EmptyResponse::default()
        }
    }
}

pub(crate) fn open_process(request: OpenProcessRequest) -> HypervisorResponse {
    let caller = NtProcess::current();

    let process = match NtProcess::from_id(request.process_id) {
        Some(process) => process,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
    };

    match request.open_type {
        ObjectOpenType::Handle => OpenObjectResponse {
            object: ObjectType::Handle(process.open_handle().unwrap().get_forget() as _),
        }
        .into_raw(),
        ObjectOpenType::Hypervisor => {
            let uid = process.nt_process as u64;
            caller
                .get_object_tracker()
                .unwrap()
                .add_open_process(process);

            OpenObjectResponse {
                object: ObjectType::Process(uid) as _,
            }
            .into_raw()
        }
    }
}
