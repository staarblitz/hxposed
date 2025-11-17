use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::process::OpenProcessResponse;
use crate::intern::instructions::{vmcall, vmcall_typed};

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OpenProcessRequest {
    pub process_id: u32,
    pub open_type: ProcessOpenType
}

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct CloseProcessRequest {
    pub addr: u64,
    pub open_type: ProcessOpenType
}

impl VmcallRequest for OpenProcessRequest {
    type Response = OpenProcessResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::open_process(),
            arg1: self.process_id as _,
            arg2: self.open_type.to_bits() as _,
            arg3: 0
        }
    }

    fn from_raw(call: HypervisorCall, arg1: u64, arg2: u64, arg3: u64) -> Self {
        Self{
            process_id: arg1 as _,
            open_type: ProcessOpenType::from_bits(arg2 as _)
        }
    }
}

impl VmcallRequest for CloseProcessRequest {
    type Response = ();

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::close_process(),
            arg1: self.addr as _,
            arg2: self.open_type.to_bits() as _,
            arg3: 0
        }
    }

    fn from_raw(call: HypervisorCall, arg1: u64, arg2: u64, arg3: u64) -> Self {
        Self{
            addr: arg1,
            open_type: ProcessOpenType::from_bits(arg2 as _)
        }
    }
}


#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ProcessOpenType {
    #[default]
    #[deprecated(note = "Has no effect.")]
    Handle = 0,
    Hypervisor = 1
}

impl ProcessOpenType {
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => Self::Handle,
            _ => Self::Hypervisor
        }
    }

    pub const fn to_bits(self) -> u16 {
        self as u16
    }
}