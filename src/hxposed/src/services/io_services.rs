use crate::nt::arch::hxfs::HxFs;
use crate::utils::intrin;
use crate::utils::intrin::{rdmsr_failsafe, wrmsr_failsafe};
use core::arch::asm;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::requests::io::{
    MsrIoRequest, MsrOperation, PrivilegedInstruction, PrivilegedInstructionRequest,
};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::io::{MsrIoResponse, PrivilegedInstructionResponse};
use hxposed_core::hxposed::responses::{HxResponse, SyscallResponse};
use crate::nt::thread::NtThread;

pub fn rw_msr(request: MsrIoRequest) -> HxResponse {
    match request.operation {
        MsrOperation::Read => match rdmsr_failsafe(request.msr) {
            Some(value) => MsrIoResponse { value }.into_raw(),
            None => HxResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
        MsrOperation::Write => match wrmsr_failsafe(request.msr, request.value) {
            Some(_) => EmptyResponse::default(),
            None => HxResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
        MsrOperation::Unknown => HxResponse::invalid_params(0)
    }
}

pub fn exec_privileged(request: PrivilegedInstructionRequest) -> HxResponse {
    match request.instruction {
        PrivilegedInstruction::Hlt => unsafe { asm!("hlt") },
        PrivilegedInstruction::MovToCr8(cr8) => unsafe {
            asm!("mov cr8, {}", in(reg) cr8);
        },
        PrivilegedInstruction::MovFromCr8(cr8) => {
            unsafe {
                asm!("mov {}, cr8", in(reg) cr8);
            }
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::MovFromCr8(cr8),
            }
            .into_raw();
        }
        PrivilegedInstruction::Lgdt(gdt) => unsafe {
            asm!("lgdt [{}]", in(reg) gdt);
        },
        PrivilegedInstruction::Lidt(idt) => unsafe {
            asm!("lidt [{}]", in(reg) idt);
        },
        PrivilegedInstruction::Sgdt(_) => {
            let table = intrin::sgdt();
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::Sgdt(table.base as _),
            }
            .into_raw();
        }
        PrivilegedInstruction::Sidt(_) => {
            let table = intrin::sidt();
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::Sidt(table.base as _),
            }
            .into_raw();
        }
        PrivilegedInstruction::MovToRFlags(rflags) => unsafe {
            let thread = NtThread::current();
            let frame = thread.get_syscall_frame();
            frame.rflags = rflags;
        },
        _ => return HxResponse::invalid_params(0),
    };

    EmptyResponse::default()
}
