use bit_field::BitField;
use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::io::{InterProcessorInterruptRequest, MsrIoRequest, MsrOperation};
use crate::services::types::cpu_fields::*;

pub struct HxCpu {}

impl HxCpu {
    pub fn read_msr(msr: u32) -> Result<u64, HypervisorError> {
        let k = MsrIoRequest {
            msr,
            value: 0,
            operation: MsrOperation::Read,
        }
        .send()?;

        Ok(k.value)
    }

    pub fn write_msr(msr: u32, value: u64) -> Result<(), HypervisorError> {
        MsrIoRequest {
            msr,
            value,
            operation: MsrOperation::Write,
        }
        .send()?;

        Ok(())
    }

    /// This doesn't hold you back from using illegal combinations
    /// Be careful, or get rekted by the silicon.
    pub fn send_ipi(
        vector: u8,
        delivery_mode: DeliveryMode,
        destination: Destination,
        trigger_mode: Option<TriggerMode>,
        level: Option<Level>,
    ) {

        let mut ipi = InterProcessorInterrupt::new()
            .with_vector(vector)
            .with_delivery_mode(delivery_mode);

        if let Some(trigger_mode) = trigger_mode {
            ipi.set_trigger_mode(trigger_mode);
        }

        if let Some(level) = level {
            ipi.set_level(level);
        }

        match destination {
            Destination::Physical(v) => {
                ipi.set_apic_id(v)
            }
            Destination::Logical(v) => {
                ipi.set_destination(v)
            }
        }

        HxCpu::write_msr(0x830, ipi.into_bits());

        Some(())
    }
}
