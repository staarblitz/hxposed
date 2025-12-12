use crate::hxposed::call::HypervisorCall;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::memory::*;
use crate::services::types::memory_fields::{MemoryPool, MemoryProtection};
use alloc::boxed::Box;
use core::mem;

#[derive(Default, Debug)]
pub struct RWProcessMemoryRequest {
    pub id: u32,
    pub address: *mut u8,
    pub count: usize,
    pub data: *mut u8,
    pub data_len: usize,
    pub operation: ProcessMemoryOperation,
}

#[derive(Default, Debug)]
pub struct ProtectProcessMemoryRequest {
    pub id: u32,
    pub address: *mut u8,
    pub protection: MemoryProtection,
}

#[derive(Default, Debug)]
pub struct AllocateMemoryRequest {
    pub size: u32,
    pub align: usize,
    pub pool: MemoryPool,
}

#[derive(Default, Debug)]
pub struct MapMemoryRequest {
    pub mdl_address: u64,
    pub map_address: u64,
    pub operation: MapMemoryOperation,
}

#[derive(Default, Debug)]
pub struct FreeMemoryRequest {
    pub mdl_address: u64,
}

impl VmcallRequest for FreeMemoryRequest {
    type Response = EmptyResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::free_mem(),
            arg1: self.mdl_address,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            mdl_address: request.arg1,
        }
    }
}

impl VmcallRequest for MapMemoryRequest {
    type Response = MapMemoryResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::mem_map(),
            arg1: self.mdl_address,
            arg2: self.map_address,
            arg3: self.operation.clone().into_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            mdl_address: request.arg1,
            map_address: request.arg2,
            operation: MapMemoryOperation::from_bits(request.arg3 as _),
        }
    }
}

impl VmcallRequest for AllocateMemoryRequest {
    type Response = AllocateMemoryResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::mem_alloc(),
            arg1: self.size as _,
            arg2: self.align as _,
            arg3: self.pool.into_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            size: request.arg1 as _,
            align: request.arg2 as _,
            pool: MemoryPool::from_bits(request.arg3 as _),
        }
    }
}

impl VmcallRequest for RWProcessMemoryRequest {
    type Response = RWProcessMemoryResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::process_vm_op(),
            arg1: self.id as _,
            arg2: self.address as _,
            arg3: self.count as _,

            extended_arg1: self.data as _,
            extended_arg2: self.data_len as _,
            extended_arg3: self.operation.clone().into_bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
            address: request.arg2 as *mut u8,
            count: request.arg3 as _,

            data: request.extended_arg1 as _,
            data_len: request.extended_arg2 as _,
            operation: ProcessMemoryOperation::from_bits(request.extended_arg3 as _),
        }
    }
}

impl VmcallRequest for ProtectProcessMemoryRequest {
    type Response = ProtectProcessMemoryResponse;

    fn into_raw(self) -> *mut HypervisorRequest {
        let raw = Box::new(HypervisorRequest {
            call: HypervisorCall::process_vm_protect(),
            arg1: self.id as _,
            arg2: self.address as _,
            arg3: self.protection.bits() as _,

            ..Default::default()
        });

        mem::forget(self);

        Box::into_raw(raw)
    }

    fn from_raw(request: &HypervisorRequest) -> Self {
        Self {
            id: request.arg1 as _,
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
