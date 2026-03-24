use crate::hypervisor::vcpu::Vmcs;
use crate::utils::intrin::{rdmsr_failsafe, wrmsr_failsafe};
use core::arch::asm;
use bit_field::BitField;
use x86::controlregs::cr3;
use x86::vmx::vmcs;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::requests::io::{
    MsrIoRequest, MsrOperation, PrivilegedInstruction, PrivilegedInstructionRequest,
};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::io::{MsrIoResponse, PrivilegedInstructionResponse};
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hypervisor::vmfs::HvFs;
use crate::utils;

pub fn rw_msr(request: MsrIoRequest) -> HypervisorResponse {
    match request.operation {
        MsrOperation::Read => match rdmsr_failsafe(request.msr) {
            Some(value) => MsrIoResponse { value }.into_raw(),
            None => HypervisorResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
        MsrOperation::Write => match wrmsr_failsafe(request.msr, request.value) {
            Some(_) => EmptyResponse::default(),
            None => HypervisorResponse::not_allowed(NotAllowedReason::AccessViolation),
        },
    }
}

pub fn exec_privileged(request: PrivilegedInstructionRequest) -> HypervisorResponse {
    match request.instruction {
        PrivilegedInstruction::Hlt => {
            const ACTIVITY_STATE_HLT: u64 = 2;
            // set the guest activity state to HLT
            Vmcs::vmwrite(vmcs::guest::ACTIVITY_STATE, ACTIVITY_STATE_HLT);
        },
        PrivilegedInstruction::MovToCr8(cr8) => {
            unsafe {
                asm!("mov cr8, {}", in(reg) cr8);
            }
        }
        PrivilegedInstruction::MovToCr3(cr3) => {
            Vmcs::vmwrite(vmcs::guest::CR3, cr3);
        }
        // unless APIC virtualization is enabled, CR8 is "passthrough"
        PrivilegedInstruction::MovFromCr8(mut cr8) => {
            unsafe {
                asm!("mov {}, cr8", out(reg) cr8);
            }

            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::MovFromCr8(cr8)
            }.into_raw();
        }
        PrivilegedInstruction::MovFromCr3(_) => {
            let cr3 = Vmcs::vmread(vmcs::guest::CR3);
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::MovFromCr3(cr3)
            }.into_raw();
        }
        PrivilegedInstruction::Lgdt(gdt, limit) => {
            Vmcs::vmwrite(vmcs::guest::GDTR_BASE, gdt);
            Vmcs::vmwrite(vmcs::guest::GDTR_LIMIT, limit);
        }
        PrivilegedInstruction::Lidt(idt, limit) => {
            Vmcs::vmwrite(vmcs::guest::IDTR_BASE, idt);
            Vmcs::vmwrite(vmcs::guest::IDTR_LIMIT, limit);
        }
        PrivilegedInstruction::Sgdt(_, _) => {
            let base = Vmcs::vmread(vmcs::guest::GDTR_BASE);
            let limit = Vmcs::vmread(vmcs::guest::GDTR_LIMIT);
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::Sgdt(base, limit as _)
            }.into_raw();
        }
        PrivilegedInstruction::Sidt(_, _) => {
            let base = Vmcs::vmread(vmcs::guest::IDTR_BASE);
            let limit = Vmcs::vmread(vmcs::guest::IDTR_LIMIT);
            return PrivilegedInstructionResponse {
                instruction: PrivilegedInstruction::Sidt(base, limit as _)
            }.into_raw();
        }
        PrivilegedInstruction::MovToRFlags(rflags) => unsafe {
            (*HvFs::get_current()).registers.rflags = rflags;
        },
        PrivilegedInstruction::Unknown => return HypervisorResponse::invalid_params(0)
    };

    EmptyResponse::default()
}
