use crate::plugins::plugin::Plugin;
use crate::write_response;
use core::sync::atomic::{AtomicPtr, Ordering};
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::{HypervisorCall, HypervisorResult};
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::VmcallRequest;
use hxposed_core::hxposed::requests::process::{CloseProcessRequest, OpenProcessRequest};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::process::OpenProcessResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use wdk_sys::ntddk::PsLookupProcessByProcessId;
use wdk_sys::{PEPROCESS, STATUS_SUCCESS};

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
        HypervisorResponse::not_allowed(NotAllowedReason::Unknown, PluginPermissions::empty())
    }
}

pub(crate) fn open_process(
    _guest: &mut dyn Guest,
    request: OpenProcessRequest,
    plugin: &'static mut Plugin,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::PROCESS_EXECUTIVE) {
        return HypervisorResponse::not_allowed(
            NotAllowedReason::MissingPermissions,
            PluginPermissions::PROCESS_EXECUTIVE,
        );
    }
    let mut process = PEPROCESS::default();

    let status = unsafe { PsLookupProcessByProcessId(request.process_id as _, &mut process) };

    if status != STATUS_SUCCESS {
        return HypervisorResponse::nt_error(status as _);
    }

    plugin.open_processes.push(AtomicPtr::new(process));

    OpenProcessResponse { addr: process as _ }.into_raw()
}
