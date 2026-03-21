use crate::nt::object::NtHandle;
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::requests::handle::{GetHandleObjectRequest, SwapHandleObjectRequest, UpgradeHandleRequest};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::handle::GetHandleObjectResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub(crate) fn upgrade_handle(request: UpgradeHandleRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let process = match process
        .get_object_tracker_unchecked()
        .get_open_process(request.process)
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
        Some(x) => x,
    };
    let handle_entry = match NtHandle::get_handle_entry(request.handle, process.get_handle_table())
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Handle),
        Some(entry) => entry,
    };

    NtHandle::upgrade_handle(handle_entry, request.access_rights);

    EmptyResponse::default()
}

pub(crate) fn swap_handle_obj(request: SwapHandleObjectRequest) -> HypervisorResponse {
    // this needs to become a macro
    let process = NtProcess::current();
    let process = match process
        .get_object_tracker_unchecked()
        .get_open_process(request.process)
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
        Some(x) => x,
    };
    let handle_entry = match NtHandle::get_handle_entry(request.handle, process.get_handle_table())
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Handle),
        Some(entry) => entry,
    };

    NtHandle::set_object_ptr(handle_entry, request.object as *mut u64);

    EmptyResponse::default()
}

pub(crate) fn get_handle_obj(request: GetHandleObjectRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let process = match process
        .get_object_tracker_unchecked()
        .get_open_process(request.process)
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Process),
        Some(x) => x,
    };
    let handle_entry = match NtHandle::get_handle_entry(request.handle, process.get_handle_table())
    {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Handle),
        Some(entry) => entry,
    };

    GetHandleObjectResponse {
        object: NtHandle::get_object_ptr::<u64>(handle_entry) as _,
        granted_access: NtHandle::get_granted_access(handle_entry),
    }.into_raw()
}