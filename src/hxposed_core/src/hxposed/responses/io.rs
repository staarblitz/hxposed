use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::io::PrivilegedInstruction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone)]
pub struct PrivilegedInstructionResponse {
    pub instruction: PrivilegedInstruction
}

#[derive(Debug, Clone)]
pub struct MsrIoResponse {
    pub value: u64,
}

impl VmcallResponse for PrivilegedInstructionResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            instruction: PrivilegedInstruction::from_bits(raw.arg1, raw.arg2, raw.arg3 as _)
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.instruction.into_raw();
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: args.0,
            arg2: args.1,
            arg3: args.2 as _,
            ..Default::default()
        }
    }
}

impl VmcallResponse for MsrIoResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self { value: raw.arg1 }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: self.value,
            ..Default::default()
        }
    }
}
