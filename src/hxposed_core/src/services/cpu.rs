use crate::error::HypervisorError;
use crate::hxposed::requests::io::{MsrIoRequest, MsrOperation};
use crate::hxposed::requests::Vmcall;

pub struct HxCpu {
    a: bool
}

impl HxCpu {
    pub fn read_msr(msr: u32) -> Result<u64, HypervisorError> {
        let k = MsrIoRequest {
            msr,
            value: 0,
            operation: MsrOperation::Read,
        }.send()?;

        Ok(k.value)
    }

    pub fn write_msr(msr: u32, value: u64) -> Result<(), HypervisorError> {
        MsrIoRequest {
            msr,
            value,
            operation: MsrOperation::Write,
        }.send()?;

        Ok(())
    }
}