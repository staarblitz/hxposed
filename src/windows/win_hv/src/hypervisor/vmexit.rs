use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::func::ServiceFunction::Authorize;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::status::HypervisorStatus;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use crate::plugins::PluginTable;
use crate::services::authorize_plugin;
use crate::{services, write_response};

///
/// # Called when a CPUID with RCX = 2009 is executed.
///
/// ## Arguments
/// guest - The trait of guest. Intel or AMD.
///
/// info - Information about the call. See [HypervisorCall].
///
/// ## Return
/// There is no return value of this function, however, the return value of the vmcall will be in RSI.
/// Which you *may* want to utilize. See documentation on GitHub page for more information about trap ABI.
///
/// ## Warning
///
/// ### We are in context of the thread that made the vmcall.
/// Functions like "IoGetCurrentProcess" returns the process that made the vmcall, not the system process.
/// (that is a good thing)
///
/// ### IRQL is above sane.
/// IRQL is 255, all interrupts are disabled. Using Zw* and other functions that ask for PASSIVE_LEVEL will only result in tears.
///
/// ### This is a VMEXIT handler
/// Don't you dare to "take your time". This interrupts the whole CPU and making the kernel scheduler forget its purpose.
///
pub(crate) fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    log::trace!("Handling vmcall function: {:?}", info.func());

    if info.func() == ServiceFunction::GetState {
        write_response(
            guest,
            StatusResponse {
                state: HypervisorStatus::SystemVirtualized,
                version: 1,
            }
                .into_raw(),
        );
        return;
    }

    let mut async_info = UnsafeAsyncInfo::default();

    let request = HypervisorRequest {
        call: info,
        arg1: guest.regs().r8,
        arg2: guest.regs().r9,
        arg3: guest.regs().r10,
        extended_arg1: guest.regs().xmm0.into(),
        extended_arg2: guest.regs().xmm1.into(),
        extended_arg3: guest.regs().xmm2.into(),
        extended_arg4: guest.regs().xmm3.into(),
    };

    if info.is_async() {
        async_info = UnsafeAsyncInfo {
            handle: guest.regs().r11,
            result_values: guest.regs().r12 as *mut _, // rsi, r8, r9, r10. total 4
        };

        log::trace!(
            "Async Handle: {:x}. Result values: {:x}",
            async_info.handle,
            async_info.result_values.addr()
        )
    }

    let plugin = match PluginTable::current() {
        None => {
            log::trace!("Plugin is not authorized.");
            if info.func() == Authorize {
                log::trace!("Authorizing...");
                authorize_plugin(guest, AuthorizationRequest::from_raw(&request));
                return;
            }
            log::warn!("Plugin tried to use HxPosed without authorizing first.");
            write_response(
                guest,
                HypervisorResponse::not_allowed(NotAllowedReason::PluginNotLoaded),
            );
            return;
        }
        Some(x) => x,
    };

    // we could actually use a bit mask that defines which category the service belongs to.
    // so we would spare ourselves from checking the func 2 times.
    // but rust enums aren't that easy, so we got this.
    // TODO: do what I said.
    match info.func() {
        ServiceFunction::OpenProcess
        | ServiceFunction::CloseProcess
        | ServiceFunction::KillProcess
        | ServiceFunction::GetProcessField
        | ServiceFunction::SetProcessField
        | ServiceFunction::GetProcessThreads => {
            services::handle_process_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::OpenThread
        | ServiceFunction::CloseThread
        | ServiceFunction::SuspendResumeThread
        | ServiceFunction::KillThread
        | ServiceFunction::GetThreadField
        | ServiceFunction::SetThreadField => {
            services::handle_thread_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::ProcessVMOperation
        | ServiceFunction::ProtectProcessMemory
        | ServiceFunction::AllocateMemory
        | ServiceFunction::MapMemory
        | ServiceFunction::FreeMemory => {
            services::handle_memory_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::OpenToken
        | ServiceFunction::CloseToken
        | ServiceFunction::GetTokenField
        | ServiceFunction::SetTokenField => {
            services::handle_security_services(guest, &request, plugin, async_info);
        }
        _ => {
            log::warn!("Unsupported vmcall: {:?}", info.func());
            write_response(
                guest,
                HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction),
            )
        }
    }
}