use crate::nt::guard::hxguard::HxGuard;
use crate::nt::process::NtProcess;
use crate::services;
use crate::utils::benchmark::CpuBenchmark;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::{HypervisorRequest};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;

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
    let process = NtProcess::current();

    match HxGuard::is_valid_caller(process.get_path_hash()) {
        true => {}
        false => {
            log::warn!("Caller failed verification.");
            return false;
        }
    }

    match process.is_hx_info_present() {
        true => {}
        false => {
            log::warn!("Caller does not have HxInfo on its EPROCESS structure.");
            guest.write_response(HypervisorResponse::not_found_what(
                NotFoundReason::HxInfo as _,
            ));
            return false;
        }
    }

    // partial uninit for performance??
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

    // we could actually use a bit mask that defines which category the service belongs to.
    // so we would spare ourselves from checking the func 2 times.
    // but rust enums aren't that easy, so we got this.
    // TODO: do what I said.
    let result = match info.func() {
        ServiceFunction::GetState => StatusResponse {
            state: HypervisorStatus::SystemVirtualized,
            version: 1,
        }
        .into_raw(),
        ServiceFunction::OpenProcess
        | ServiceFunction::CloseProcess
        | ServiceFunction::KillProcess
        | ServiceFunction::GetProcessField
        | ServiceFunction::SetProcessField
        | ServiceFunction::GetProcessThreads => services::handle_process_services(&request),
        ServiceFunction::OpenThread
        | ServiceFunction::CloseThread
        | ServiceFunction::SuspendResumeThread
        | ServiceFunction::KillThread
        | ServiceFunction::GetThreadField
        | ServiceFunction::SetThreadField => services::handle_thread_services(&request),
        ServiceFunction::GetSetPageAttribute
        | ServiceFunction::AllocateMemory
        | ServiceFunction::FreeMemory
        | ServiceFunction::MapVaToPa => services::handle_memory_services(&request),
        ServiceFunction::RegisterNotifyEvent | ServiceFunction::UnregisterNotifyEvent => {
            services::handle_callback_services(&request)
        }
        ServiceFunction::OpenToken
        | ServiceFunction::CloseToken
        | ServiceFunction::GetTokenField
        | ServiceFunction::SetTokenField => services::handle_security_services(&request),
        ServiceFunction::MsrIo => services::handle_cpu_io_services(&request),
        _ => {
            log::warn!("Unsupported vmcall: {:?}", info.func());
            HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction)
        }
    };

    guest.write_response(result);
    true
}
