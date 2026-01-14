use wdk_sys::ntddk::IoGetCurrentProcess;
use crate::{services};
use hv::hypervisor::host::Guest;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::process::NtProcess;

///
/// # Called when a CPUID with RCX = 2009 is executed.
///
/// ## Arguments
/// guest - The trait of guest. Intel or AMD.
///
/// info - Information about the call. See [HypervisorCall].
///
/// ## Return
/// If true, that means HxPosed handled the call.
/// If false, that means HxPosed did NOT handle the call on purpose, and this call should be treated as a CPUID.
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
pub(crate) fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) -> bool {
    // check
    {
        let process = NtProcess::current();
        match HxGuard::is_valid_caller(process.get_path_hash()) {
            true => {}
            false => {
                log::warn!("Caller failed verification.");
                return false
            }
        }
    }


    log::trace!("Handling vmcall function: {:?}", info.func());

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
            process: unsafe {
                IoGetCurrentProcess()
            } as _,
        };

        log::trace!(
            "Async Handle: {:x}. Result values: {:x}",
            async_info.handle,
            async_info.result_values.addr()
        )
    }

    // we could actually use a bit mask that defines which category the service belongs to.
    // so we would spare ourselves from checking the func 2 times.
    // but rust enums aren't that easy, so we got this.
    // TODO: do what I said.
    let result = match info.func() {
        ServiceFunction::GetState => {
            StatusResponse {
                state: HypervisorStatus::SystemVirtualized,
                version: 1,
            }
                .into_raw()
        },
        ServiceFunction::OpenProcess
        | ServiceFunction::CloseProcess
        | ServiceFunction::KillProcess
        | ServiceFunction::GetProcessField
        | ServiceFunction::SetProcessField
        | ServiceFunction::GetProcessThreads => {
            services::handle_process_services(&request, async_info)
        }
        ServiceFunction::OpenThread
        | ServiceFunction::CloseThread
        | ServiceFunction::SuspendResumeThread
        | ServiceFunction::KillThread
        | ServiceFunction::GetThreadField
        | ServiceFunction::SetThreadField => {
            services::handle_thread_services(&request, async_info)
        }
        ServiceFunction::ProcessVMOperation
        | ServiceFunction::ProtectProcessMemory
        | ServiceFunction::AllocateMemory
        | ServiceFunction::MapMemory
        | ServiceFunction::FreeMemory => {
            services::handle_memory_services(&request, async_info)
        }
        ServiceFunction::OpenToken
        | ServiceFunction::CloseToken
        | ServiceFunction::GetTokenField
        | ServiceFunction::SetTokenField => {
            services::handle_security_services(&request, async_info)
        }
        ServiceFunction::RegisterNotifyEvent
        | ServiceFunction::AwaitNotifyEvent
        | ServiceFunction::UnregisterNotifyEvent => {
            services::handle_callback_services(&request, async_info)
        }
        _ => {
            log::warn!("Unsupported vmcall: {:?}", info.func());
            HypervisorResponse::not_found_what(
                NotFoundReason::ServiceFunction,
            )
        }
    };

    guest.write_response(result);

    true
}
