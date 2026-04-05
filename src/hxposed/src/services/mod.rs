use hxposed_core::hxposed::call::HxCall;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::requests::{HxRequest, SyscallRequest};
use hxposed_core::hxposed::requests::handle::*;
use hxposed_core::hxposed::requests::io::*;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::requests::notify::*;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::{HxResponse, SyscallResponse};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::status::HypervisorStatus;
use crate::hyper_row;
use crate::nt::arch::hxfs::{HxFs, Registers};
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::utils::logger::{HxLogger, LogEvent, LogType};

pub mod callback_services;
pub mod io_services;
pub mod memory_services;
pub mod process_services;
pub mod security_services;
pub mod thread_services;
pub mod handle_services;


pub const INV: fn(&HxRequest) -> HxResponse = invalid_handler;
pub type SyscallHandler = fn(&HxRequest) -> HxResponse;
fn invalid_handler(_req: &HxRequest) -> HxResponse {
    HxResponse::not_found_what(NotFoundReason::ServiceFunction)
}

const DISPATCH_TABLE_MAX: usize = 8;
#[allow(unused)]
static DISPATCH_TABLE: [[SyscallHandler; 16]; DISPATCH_TABLE_MAX] = [
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
        |x| { memory_services::map_va_to_pa(MapRmdRequest::from_raw(x)) },
        |x| { memory_services::translate_address(TranslateAddressRequest::from_raw(x)) },
        |x| { memory_services::describe_memory(DescribeMemoryRequest::from_raw(x)) }
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
    hyper_row!(
        |x| { io_services::rw_msr(MsrIoRequest::from_raw(x)) },
        |x| { io_services::exec_privileged(PrivilegedInstructionRequest::from_raw(x)) }
    ),
    hyper_row!(
        |x| { handle_services::upgrade_handle(UpgradeHandleRequest::from_raw(x)) },
        |x| { handle_services::get_handle_obj(GetHandleObjectRequest::from_raw(x)) },
        |x| { handle_services::swap_handle_obj(SwapHandleObjectRequest::from_raw(x)) }

    ),
];

#[unsafe(no_mangle)]
pub(crate) fn syscall_handler(registers: &mut Registers) {
    let info = HxCall::from_bits(registers.rsi);

    let process = NtThread::current();
    // SAFETY: very unsafe(ish)
    // we use the registers pointer because we know it lives longer than this function (its allocated by previous caller on stack).
    // when we ret and dispatch, the lifetime of registers end. until then, the lifetime is correct.
    process.set_syscall_frame(registers);

    // partial uninit for performance??
    let mut request = HxRequest {
        call: info,
        arg1: registers.r8,
        arg2: registers.r9,
        arg3: registers.r10,
        ..Default::default()
    };

    HxLogger::serial_log(LogType::Trace, LogEvent::SystemCall(
        request.call.into_bits(),
        request.arg1,
        request.arg2,
        request.arg3,
    ));

    if request.call.extended_args_present() {
        request.extended_arg1 = registers.xmm0;
        request.extended_arg2 = registers.xmm1;
        request.extended_arg3 = registers.xmm2;
        request.extended_arg4 = registers.xmm3;
    }

    let function = info.func().into_bits() as usize;
    const CATEGORY_MASK: usize = 0xF0;
    const FUNCTION_MASK: usize = 0x0F;

    let category = (function & CATEGORY_MASK) >> 4;
    let func = function & FUNCTION_MASK;

    if core::intrinsics::unlikely(category >= DISPATCH_TABLE_MAX) {
        registers.write_response(HxResponse::not_found_what(
            NotFoundReason::ServiceFunction,
        ));
        return;
    }
    
    let result = DISPATCH_TABLE[category][func](&request);

    HxLogger::serial_log(LogType::Trace, LogEvent::CallResult(result.arg1, result.arg2, result.arg3));

    registers.write_response(result);
}