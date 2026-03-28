use crate::hxposed::call::HxResult;
use crate::hxposed::requests::io::PrivilegedInstruction;
use crate::hxposed::responses::{HxResponse, SyscallResponse};

#[derive(Clone)]
pub struct PrivilegedInstructionResponse {
    pub instruction: PrivilegedInstruction
}

#[derive(Debug, Clone)]
pub struct MsrIoResponse {
    pub value: u64,
}

impl SyscallResponse for PrivilegedInstructionResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            instruction: PrivilegedInstruction::from_bits(raw.arg1, raw.arg2)
        }
    }

    fn into_raw(self) -> HxResponse {
        let args = self.instruction.into_raw();
        HxResponse {
            result: HxResult::ok(),
            arg1: args.0,
            arg2: args.1,
            ..Default::default()
        }
    }
}

impl SyscallResponse for MsrIoResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self { value: raw.arg1 }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.value,
            ..Default::default()
        }
    }
}
