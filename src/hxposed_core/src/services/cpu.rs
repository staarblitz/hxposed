use bit_field::BitField;
use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::io::{MsrIoRequest, MsrOperation, PrivilegedInstruction, PrivilegedInstructionRequest};
use crate::hxposed::responses::HypervisorResponse;
use crate::hxposed::utils::transaction::Transaction;
use crate::services::types::cpu_fields::*;

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
    /// * [`HypervisorError::NotAllowed`] - If MSR is inaccessible.
    pub fn read_msr(msr: u32) -> Result<u64, HypervisorError> {
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
    /// * [`HypervisorError::NotAllowed`] - If MSR is inaccessible or readonly.
    pub fn write_msr(msr: u32, value: u64) -> Result<(), HypervisorError> {
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
    pub fn execute_privileged(instruction: PrivilegedInstruction) -> Result<PrivilegedInstruction, HypervisorError> {
        let k = PrivilegedInstructionRequest {
            instruction
        }.send()?;

        Ok(k.instruction)
    }

    /// # Enable Preemption
    ///
    /// Does the following:
    /// - Writes old value to `IA32_TSC_DEADLINE`.
    /// - Sets interrupt flag via `sti`.
    /// - Sets IRQL back to `PASSIVE_LEVEL`.
    /// - This function is NOT transactional.
    ///
    /// ## Remarks
    /// - Same as [`Self::disable_preemption`].
    pub fn enable_preemption(old_tsc: u64) -> Result<(), HypervisorError> {
        Self::write_msr(0x6E0, old_tsc)?;

        Self::execute_privileged(PrivilegedInstruction::Sti)?;

        Self::execute_privileged(PrivilegedInstruction::MovToCr8(0))?;
        Ok(())
    }

    /// # Disable Preemption
    ///
    /// Does the following:
    /// - Writes 0 to `IA32_TSC_DEADLINE` in purpose of disabling preemption.
    /// - Clears interrupt flags via `cli`.
    /// - Sets IRQL to `HIGH_LEVEL`.
    /// - This function is transactional. If any of these fail, all the changes will be reverted.
    ///
    /// ## Remarks
    /// - Execution can still be interrupted due to IPIs and NMIs and so on and so forth.
    /// - Exceptions will still route to IDT.
    /// - It's strongly recommended for you to NOT do this. Windows NT may freak out when it sees this conditions on user-space code.
    /// And let me tell you, that won't end well.
    ///
    /// ## Return
    /// * [`u64`] - Previous value of `IA32_TSC_DEADLINE` to be used with [`Self::enable_preemption`].
    pub fn disable_preemption() -> Result<u64, HypervisorError> {
        let mut transaction = Transaction::new();
        let old_deadline = Self::read_msr(0x6E0)?;

        Self::write_msr(0x6E0, 0)?;
        transaction.enlist(move || {
            Self::write_msr(0x6E0, old_deadline).unwrap();
        });

        Self::execute_privileged(PrivilegedInstruction::Cli)?;
        transaction.enlist(|| {
            Self::execute_privileged(PrivilegedInstruction::Sti).unwrap();
        });

        Self::execute_privileged(PrivilegedInstruction::MovToCr8(32))?;
        transaction.enlist(|| {
            Self::execute_privileged(PrivilegedInstruction::MovToCr8(0)).unwrap();
        });

        transaction.commit();
        Ok(old_deadline)
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
    ) -> Result<(), HypervisorError> {
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

        if let Some(id) = apic_id{
            ipi.set_destination_mode(DestinationMode::Logical);
            ipi.set_apic_id(id);
        }

        HxCpu::write_msr(0x830, ipi.into_bits())
    }
}
