use crate::hyper_row;
use crate::hypervisor::vcpu::Vmcs;
use crate::hypervisor::vmfs::HvFs;
use crate::nt::process::NtProcess;
use crate::services::*;
use bit_field::BitField;
use hxposed_core::error::HypervisorError;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotFoundReason;
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
use x86::controlregs::{cr4, cr4_write, xcr0_write, Cr4, Xcr0};
use x86::cpuid::cpuid;
use x86::msr::{rdmsr, wrmsr};
use x86::vmx::vmcs;

#[unsafe(no_mangle)]
pub extern "C" fn vmexit_handler() -> u64 {
    const VMX_EXIT_REASON_VMCALL: u64 = 18;
    const VMX_EXIT_REASON_CPUID: u64 = 10;
    const VMX_EXIT_REASON_RDMSR: u64 = 31;
    const VMX_EXIT_REASON_WRMSR: u64 = 32;
    const VMX_EXIT_REASON_XSETBV: u64 = 55;

    let vcpu = unsafe { &mut *HvFs::get_current() };
    let exit_reason = Vmcs::vmread(vmcs::ro::EXIT_REASON);
    let qualification = Vmcs::vmread(vmcs::ro::EXIT_QUALIFICATION);
    let rip = Vmcs::vmread(vmcs::guest::RIP);
    let next_rip = rip + Vmcs::vmread(vmcs::ro::VMEXIT_INSTRUCTION_LEN);

    log::info!("VmExit reason: {exit_reason:x}");
    log::info!("RIP: {rip:x}, NRIP: {next_rip:x}");
    log::info!("VCPU: {:x}", vcpu as *mut _ as u64);

    match exit_reason {
        VMX_EXIT_REASON_CPUID => {
            const HV_CPUID_INTERFACE: u32 = 0x4000_0001;
            let leaf = vcpu.registers.rax as u32;
            let sub_leaf = vcpu.registers.rcx as u32;

            if vcpu.registers.rcx == 0x2009 {
                match vmcall_handler(vcpu) {
                    true => {
                        vcpu.registers.rcx = 0x2009;
                        return next_rip;
                    }
                    false => {}
                }
            }

            let mut cpuid_result = cpuid!(leaf, sub_leaf);
            if leaf == 1 {
                // clear vt-x supported bit to prevent other hypervisors from using it.
                cpuid_result.ecx = *cpuid_result.ecx.set_bit(5, false);

                cpuid_result.ecx = *cpuid_result.ecx.set_bit(31, false);
            } else if leaf == HV_CPUID_INTERFACE {
                // to support hyper-v enlightenment, we must pass the actual rax
                cpuid_result.eax = 0;
            }

            vcpu.registers.rax = u64::from(cpuid_result.eax);
            vcpu.registers.rbx = u64::from(cpuid_result.ebx);
            vcpu.registers.rcx = u64::from(cpuid_result.ecx);
            vcpu.registers.rdx = u64::from(cpuid_result.edx);
        }
        VMX_EXIT_REASON_WRMSR => {
            let msr = vcpu.registers.rcx as u32;
            let value =
                (vcpu.registers.rax & 0xffff_ffff) | ((vcpu.registers.rdx & 0xffff_ffff) << 32);
            unsafe { wrmsr(msr, value) };
        }
        VMX_EXIT_REASON_RDMSR => {
            let msr = vcpu.registers.rcx as u32;
            let value = unsafe { rdmsr(msr) };
            vcpu.registers.rax = value & 0xffff_ffff;
            vcpu.registers.rdx = value >> 32;
        }
        VMX_EXIT_REASON_XSETBV => {
            //let xcr: u32 = vcpu.registers.rcx as u32;
            let value =
                (vcpu.registers.rax & 0xffff_ffff) | ((vcpu.registers.rdx & 0xffff_ffff) << 32);
            let value = Xcr0::from_bits(value).unwrap();
            unsafe {
                cr4_write(cr4() | Cr4::CR4_ENABLE_OS_XSAVE);
                xcr0_write(value);
            }
        }
        _ => unreachable!(),
    }

    next_rip
}

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
pub(crate) fn vmcall_handler(guest: &mut HvFs) -> bool {
    let info = HypervisorCall::from_bits(guest.registers.rsi);

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
        arg1: guest.registers.r8,
        arg2: guest.registers.r9,
        arg3: guest.registers.r10,
        ..Default::default()
    };

    log::info!(
        "Hypercall:\n Call: {:?}\nArg1: {:x}\nArg2: {:x}\nArg3: {:x}",
        request.call,
        request.arg1,
        request.arg2,
        request.arg3
    );

    if request.call.extended_args_present() {
        request.extended_arg1 = guest.registers.xmm0.into();
        request.extended_arg2 = guest.registers.xmm1.into();
        request.extended_arg3 = guest.registers.xmm2.into();
        request.extended_arg4 = guest.registers.xmm3.into();

        log::info!(
            "Hypercall has extended arguments.\nEArg1: {:x}, EArg2: {:x}\nEArg3: {:x}\nEArg4: {:x}",
            request.extended_arg1,
            request.extended_arg2,
            request.extended_arg3,
            request.extended_arg4
        );
    }

    let function = info.func().into_bits() as usize;
    let category = function >> 4;
    let func = function & 0b_1111;
    if core::intrinsics::unlikely(category > DISPATCH_TABLE_MAX - 1) {
        guest.write_response(HypervisorResponse::not_found_what(
            NotFoundReason::ServiceFunction,
        ));
        return true;
    }

    log::info!("Dispatching call category: {}, id: {}", category, func);

    let result = DISPATCH_TABLE[category][func](&request);

    log::info!(
        "Error of call: {}",
        HypervisorError::from_response(result.clone())
    );
    log::info!(
        "Result of\nArg1: {:x}\nArg2: {:x}\nArg3: {:x}",
        result.arg1,
        result.arg2,
        result.arg3
    );

    guest.write_response(result);
    true
}
