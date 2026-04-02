use crate::hxposed::call::HxCall;
use crate::hxposed::requests::{HxRequest, SyscallRequest};
use crate::hxposed::responses::io::*;

#[derive(Debug)]
pub struct PrivilegedInstructionRequest {
    pub instruction: PrivilegedInstruction
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PrivilegedInstruction {
    Hlt,
    /// # Extremely dangerous
    /// HxPosed expects to be in PASSIVE_LEVEL. If you make another call without setting it to PASSIVE_LEVEL first, we have a problem.
    MovToCr8(u64),
    MovToCr3(u64),
    MovFromCr8(u64),
    MovFromCr3(u64),
    Lgdt(u64),
    Lidt(u64),
    Sgdt(u64),
    Sidt(u64),
    MovToRFlags(u64),
    Unknown
}

impl PrivilegedInstruction {
    pub const fn into_raw(self) -> (u64, u64) {
        match self {
            PrivilegedInstruction::Hlt => (0, 0),
            PrivilegedInstruction::MovToCr8(x) => (1, x),
            PrivilegedInstruction::MovToCr3(x) => (2, x),
            PrivilegedInstruction::MovFromCr8(x) => (3, x),
            PrivilegedInstruction::MovFromCr3(x) => (4, x),
            PrivilegedInstruction::Lgdt(x,) => (5, x),
            PrivilegedInstruction::Lidt(x) => (6, x),
            PrivilegedInstruction::Sgdt(x) => (7, x),
            PrivilegedInstruction::Sidt(x) => (8, x),
            PrivilegedInstruction::MovToRFlags(x) => (9, x),
            PrivilegedInstruction::Unknown => (u64::MAX, u64::MAX),
        }
    }

    pub const fn from_bits(mnemonic: u64, arg: u64) -> Self {
        match mnemonic {
            0 => PrivilegedInstruction::Hlt,
            1 => PrivilegedInstruction::MovToCr8(arg),
            2 => PrivilegedInstruction::MovToCr3(arg),
            3 => PrivilegedInstruction::MovFromCr8(arg),
            4 => PrivilegedInstruction::MovFromCr3(arg),
            5 => PrivilegedInstruction::Lgdt(arg),
            6 => PrivilegedInstruction::Lidt(arg),
            7 => PrivilegedInstruction::Sgdt(arg),
            8 => PrivilegedInstruction::Sidt(arg),
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct MsrIoRequest {
    pub msr: u32,
    pub value: u64,
    pub operation: MsrOperation
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MsrOperation {
    Read,
    Write,
    Unknown
}

impl MsrOperation {
    pub const fn into_bits(self) -> u64 {
        match self {
            MsrOperation::Read => 0,
            MsrOperation::Write => 1,
            MsrOperation::Unknown => u64::MAX
        }
    }

    pub const fn from_bits(bits: u64) -> Self {
        match bits {
            0 => MsrOperation::Read,
            1 => MsrOperation::Write,
            _ => MsrOperation::Unknown
        }
    }
}

impl SyscallRequest for PrivilegedInstructionRequest {
    type Response = PrivilegedInstructionResponse;

    fn into_raw(self) -> HxRequest {
        let args = self.instruction.into_raw();
        HxRequest {
            call: HxCall::exec_priv(),
            arg1: args.0,
            arg2: args.1,
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            instruction: PrivilegedInstruction::from_bits(request.arg1, request.arg2)
        }
    }
}

impl SyscallRequest for MsrIoRequest {
    type Response = MsrIoResponse;

    fn into_raw(self) -> HxRequest {
        HxRequest {
            call: HxCall::msr_io(),
            arg1: self.msr as _,
            arg2: self.value as _,
            arg3: self.operation.into_bits(),
            ..Default::default()
        }
    }

    fn from_raw(request: &HxRequest) -> Self {
        Self {
            msr: request.arg1 as _,
            value: request.arg2 as _,
            operation: MsrOperation::from_bits(request.arg3),
        }
    }
}