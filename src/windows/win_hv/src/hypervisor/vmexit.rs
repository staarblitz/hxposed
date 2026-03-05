use core::arch::asm;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::process::NtProcess;
use crate::services::*;
use crate::utils::benchmark::CpuBenchmark;
use crate::{hyper_row, services};
use bit_field::BitField;
use hv::hypervisor::host::Guest;
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::io::*;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::requests::notify::*;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;

pub const INV: fn(&HypervisorRequest) -> HypervisorResponse = invalid_handler;
pub type VmcallHandler = fn(&HypervisorRequest) -> HypervisorResponse;
fn invalid_handler(_req: &HypervisorRequest) -> HypervisorResponse {
    HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction)
}

const DISPATCH_TABLE_MAX: usize = 7;
#[allow(unused)]
static DISPATCH_TABLE: [[VmcallHandler; 16]; DISPATCH_TABLE_MAX] = [
    hyper_row!(|_| {
        StatusResponse {
            state: HypervisorStatus::SystemVirtualized,
            version: 1,
        }
        .into_raw()
    }),
    hyper_row!(
        |x| { process_services::open_process(OpenProcessRequest::from_raw(x)) },
        |x| { process_services::close_process(CloseProcessRequest::from_raw(x)) },
        |x| { process_services::get_process_field_sync(GetProcessFieldRequest::from_raw(x)) },
        |x| { process_services::set_process_field_sync(SetProcessFieldRequest::from_raw(x)) }
    ),
    hyper_row!(
        |x| {
            callback_services::register_callback_receiver(RegisterNotifyHandlerRequest::from_raw(x))
        },
        |x| {
            callback_services::unregister_callback_receiver(
                UnregisterNotifyHandlerRequest::from_raw(x),
            )
        }
    ),
    hyper_row!(
        |x| { memory_services::allocate_memory(AllocateMemoryRequest::from_raw(x)) },
        |x| { memory_services::free_memory(FreeMemoryRequest::from_raw(x)) },
        |x| { memory_services::get_set_page_attribute(PageAttributeRequest::from_raw(x)) },
        |x| { memory_services::map_va_to_pa(MapVaToPaRequest::from_raw(x)) }
    ),
    hyper_row!(
        |x| { thread_services::open_thread_sync(OpenThreadRequest::from_raw(x)) },
        |x| { thread_services::close_thread_sync(CloseThreadRequest::from_raw(x)) },
        |x| { thread_services::get_thread_field_sync(GetThreadFieldRequest::from_raw(x)) },
        |x| { thread_services::set_thread_field_sync(SetThreadFieldRequest::from_raw(x)) }
    ),
    hyper_row!(
        |x| { security_services::open_token_sync(OpenTokenRequest::from_raw(x)) },
        |x| { security_services::close_token_sync(CloseTokenRequest::from_raw(x)) },
        |x| { security_services::get_token_field_sync(GetTokenFieldRequest::from_raw(x)) },
        |x| { security_services::set_token_field_sync(SetTokenFieldRequest::from_raw(x)) }
    ),
    hyper_row!(|x| { io_services::rw_msr(MsrIoRequest::from_raw(x)) }),
];

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
    let mut request = HypervisorRequest {
        call: info,
        arg1: guest.regs().r8,
        arg2: guest.regs().r9,
        arg3: guest.regs().r10,
        ..Default::default()
    };

    if request.call.extended_args_present() {
        request.extended_arg1 = guest.regs().xmm0.into();
        request.extended_arg2 = guest.regs().xmm1.into();
        request.extended_arg3 = guest.regs().xmm2.into();
        request.extended_arg4 = guest.regs().xmm3.into();
    }

    let function_id = info.func().into_bits() as usize;
    let category = function_id >> 4;
    let func = function_id & 0b_1111;
    if core::intrinsics::unlikely(category > DISPATCH_TABLE_MAX) {
        guest.write_response(HypervisorResponse::not_found_what(
            NotFoundReason::ServiceFunction,
        ));
        return true;
    }

    let result = DISPATCH_TABLE[category][func](&request);

    guest.write_response(result);
    true
}
