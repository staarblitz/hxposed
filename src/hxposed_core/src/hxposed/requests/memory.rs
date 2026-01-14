use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::memory::*;
use crate::hxposed::{MdlObject, ProcessObject};
use crate::services::types::memory_fields::{MemoryPool, MemoryProtection};

#[derive(Default, Debug)]
pub struct RWProcessMemoryRequest {
    pub process: ProcessObject,
    pub address: usize,
    pub count: usize,
    pub data: usize,
    pub data_len: usize,
    pub operation: ProcessMemoryOperation,
}

#[derive(Default, Debug)]
pub struct ProtectProcessMemoryRequest {
    pub process: ProcessObject,
    pub address: usize,
    pub protection: MemoryProtection,
}

#[derive(Default, Debug)]
pub struct AllocateMemoryRequest {
    pub size: u32,
    pub underlying_pages: usize,
    pub pool: MemoryPool,
}

#[derive(Default, Debug)]
pub struct MapMemoryRequest {
    pub mdl: MdlObject,
    pub map_address: u64,
    pub operation: MapMemoryOperation,
    pub process: ProcessObject,
}

#[derive(Default, Debug)]
pub struct FreeMemoryRequest {
    pub mdl: MdlObject,
}

impl VmcallRequest for FreeMemoryRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::free_mem(),
            arg1: self.mdl,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self { mdl: request.arg1 }
    }
}

impl VmcallRequest for MapMemoryRequest {
    type Response = MapMemoryResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::mem_map(),
            arg1: self.mdl,
            arg2: self.map_address,
            arg3: self.operation.clone().into_bits() as _,
            extended_arg1: self.process as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            mdl: request.arg1,
            map_address: request.arg2,
            operation: MapMemoryOperation::from_bits(request.arg3 as _),
            process: request.extended_arg1 as _,
        }
    }
}

impl VmcallRequest for AllocateMemoryRequest {
    type Response = AllocateMemoryResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::mem_alloc(),
            arg1: self.size as _,
            arg2: self.underlying_pages as _,
            arg3: self.pool.into_bits() as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            size: request.arg1 as _,
            underlying_pages: request.arg2 as _,
            pool: MemoryPool::from_bits(request.arg3 as _),
        }
    }
}

impl VmcallRequest for RWProcessMemoryRequest {
    type Response = RWProcessMemoryResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::process_vm_op(),
            arg1: self.process as _,
            arg2: self.address as _,
            arg3: self.count as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,
            extended_arg3: self.operation.clone().into_bits() as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            address: request.arg2 as _,
            count: request.arg3 as _,

            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
            operation: ProcessMemoryOperation::from_bits(request.extended_arg3 as _),
        }
    }
}

impl VmcallRequest for ProtectProcessMemoryRequest {
    type Response = ProtectProcessMemoryResponse;

    fn into_raw(self) -> HypervisorRequest {
        HypervisorRequest {
            call: HypervisorCall::process_vm_protect(),
            arg1: self.process as _,
            arg2: self.address as _,
            arg3: self.protection.bits() as _,

            ..Default::default()
        }
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            process: request.arg1 as _,
            address: request.arg2 as _,
            protection: MemoryProtection::from_bits(request.arg3 as _).unwrap(),
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum MapMemoryOperation {
    #[default]
    Map,
    Unmap,
}

impl MapMemoryOperation {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Map,
            1 => Self::Unmap,
            _ => Self::Map,
        }
    }
}

#[derive(Clone, Default, Eq, PartialEq, Hash, Debug)]
pub enum ProcessMemoryOperation {
    #[default]
    Read,
    Write,
}

impl ProcessMemoryOperation {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Read,
            1 => Self::Write,
            _ => Self::Read,
        }
    }
}
