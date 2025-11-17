use crate::plugins::plugin::Plugin;
use crate::services::require_perm;
use crate::write_response;
use core::sync::atomic::{AtomicPtr, Ordering};
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::requests::VmcallRequest;
use hxposed_core::hxposed::requests::process::{CloseProcessRequest, OpenProcessRequest};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use wdk_sys::ntddk::PsLookupProcessByProcessId;
use wdk_sys::{PEPROCESS, STATUS_SUCCESS};

pub(crate) fn close_process(
    guest: &mut dyn Guest,
    call: HypervisorCall,
    args: (u64, u64, u64),
    plugin: &'static mut Plugin,
) {
    let close_req = CloseProcessRequest::from_raw(call, args.0, args.1, args.2);

    if let Some((index, _)) = plugin
        .open_processes
        .iter()
        .enumerate()
        .find(|(_, x)| x.load(Ordering::Relaxed).addr() as u64 == close_req.addr)
    {
        plugin.open_processes.remove(index);
    } else {
        // this is weird. a plugin should never attempt to close a process it has never opened in the first place.
        // abuse detected. blacklist the plugin (soon)
        write_response(
            guest,
            HypervisorResponse::not_allowed(NotAllowedReason::Unknown, PluginPermissions::empty()),
        );
    }
}

pub(crate) fn open_process(
    guest: &mut dyn Guest,
    call: HypervisorCall,
    args: (u64, u64, u64),
    plugin: &'static mut Plugin,
) {
    if !require_perm(
        guest,
        plugin.permissions,
        PluginPermissions::PROCESS_EXECUTIVE,
    ) {
        return;
    }

    let open_req = OpenProcessRequest::from_raw(call, args.0, args.1, args.2);

    let mut process = PEPROCESS::default();

    let status = unsafe { PsLookupProcessByProcessId(open_req.process_id as _, &mut process) };

    if status != STATUS_SUCCESS {
        write_response(guest, HypervisorResponse::nt_error(status as _));
        return;
    }

    plugin.open_processes.push(AtomicPtr::new(process))
}
