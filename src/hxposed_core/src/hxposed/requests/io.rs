use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::io::MsrIoResponse;

#[derive(Debug)]
pub struct MsrIoRequest {
    pub msr: u32,
    pub value: u64,
    pub operation: MsrOperation
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MsrOperation {
    Read,
    Write
}

impl MsrOperation {
    pub const fn into_bits(self) -> u64 {
        match self {
            MsrOperation::Read => 0,
            MsrOperation::Write => 1,
        }
    }

    pub const fn from_bits(bits: u64) -> Self {
        match bits {
            0 => MsrOperation::Read,
            1 => MsrOperation::Write,
            _ => unreachable!()
        }
    }
}

impl VmcallRequest for MsrIoRequest {
    type Response = MsrIoResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::msr_io(),
            arg1: self.msr as _,
            arg2: self.value as _,
            arg3: self.operation.into_bits(),
            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            msr: request.arg1 as _,
            value: request.arg2 as _,
            operation: MsrOperation::from_bits(request.arg3),
        }
    }
}