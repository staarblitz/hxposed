use crate::error::HxError;
use crate::hxposed::requests::Syscall;
use crate::hxposed::requests::io::{
    MsrIoRequest, MsrOperation, PrivilegedInstruction, PrivilegedInstructionRequest,
};
use crate::hxposed::responses::HxResponse;
use crate::hxposed::utils::transaction::Transaction;
use crate::services::types::cpu_fields::*;
use bit_field::BitField;
use std::arch::asm;

pub struct HxCpu {}

impl HxCpu {
    /// # Read MSR
    ///
    /// Reads MSR from current core.
    ///
    /// ## Remarks
    /// - This function is SAFE. It will not crash the system if MSR doesn't exist.
    ///
    /// ## Return
    /// * [`u64`] - If MSR is accessible and read.
    /// * [`HxError::NotAllowed`] - If MSR is inaccessible.
    pub fn read_msr(msr: u32) -> Result<u64, HxError> {
        let k = MsrIoRequest {
            msr,
            value: 0,
            operation: MsrOperation::Read,
        }
        .send()?;

        Ok(k.value)
    }

    /// # Write MSR
    ///
    /// Writes MSR from current core.
    ///
    /// ## Remarks
    /// - This function is SAFE. It will not crash the system if MSR doesn't exist or is readonly.
    ///
    /// ## Return
    /// * [`()`] - If MSR was accessible and written.
    /// * [`HxError::NotAllowed`] - If MSR is inaccessible or readonly.
    pub fn write_msr(msr: u32, value: u64) -> Result<(), HxError> {
        MsrIoRequest {
            msr,
            value,
            operation: MsrOperation::Write,
        }
        .send()?;

        Ok(())
    }

    /// # Execute Privileged Instruction
    ///
    /// Executes any privileged instruction in context of current core.
    ///
    /// If you want to read/write to MSRs, check out [`Self::read_msr`] and [`Self::write_msr`].
    ///
    /// ## Remarks
    /// - Any registers (e.g. CR3) correspond to guest registers, not host ones.
    /// - Unlike other abstractions, there is no safety for execution. If you mess up, you mess up.
    pub fn execute_privileged(
        instruction: PrivilegedInstruction,
    ) -> Result<PrivilegedInstruction, HxError> {
        let k = PrivilegedInstructionRequest { instruction }.send()?;

        Ok(k.instruction)
    }

    /// # Set RFLAGS.IOPL
    ///
    /// Sets the RFLAGS I/O Privilege Level field to allow port i/o and execution of cli/sti
    ///
    pub fn set_iopl(new: u8) -> Result<(), HxError> {
        let mut rflags = 0u64;
        unsafe { asm!("pushfq", "pop rax", out("rax") rflags) }
        Self::execute_privileged(PrivilegedInstruction::MovToRFlags(
            *rflags.set_bits(12..13, new as _),
        ))
        .map(|_| ())
    }

    /// # Send Inter Processor Interrupt
    ///
    /// Sends an IPI using xAPIC2. Older APICs are not supported.
    ///
    /// ## Remarks
    /// - This function doesn't include all checks in Intel SDM Volume 3 Chapter 12.6 "Issuing Interprocessor Interrupts".
    /// - So be careful.
    pub fn send_ipi(
        vector: u8,
        delivery_mode: DeliveryMode,
        destination_shorthand: DestinationShorthand,
        apic_id: Option<u32>,
        trigger_mode: Option<TriggerMode>,
        level: Option<Level>,
    ) -> Result<(), HxError> {
        let mut ipi = InterProcessorInterrupt::new()
            .with_vector(vector)
            .with_delivery_mode(delivery_mode)
            .with_destination(destination_shorthand);

        if let Some(trigger_mode) = trigger_mode {
            ipi.set_trigger_mode(trigger_mode);
        }

        if let Some(level) = level {
            ipi.set_level(level);
        }

        if let Some(id) = apic_id {
            ipi.set_destination_mode(DestinationMode::Logical);
            ipi.set_apic_id(id);
        }

        HxCpu::write_msr(0x830, ipi.into_bits())
    }
}
