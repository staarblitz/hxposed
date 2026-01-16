use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::MdlObject;
use crate::services::types::memory_fields::MemoryProtection;

#[derive(Clone, Default, Debug)]
pub struct RWProcessMemoryResponse {
    pub bytes_processed: usize,
}

#[derive(Clone, Default)]
pub struct ProtectProcessMemoryResponse {
    pub old_protection: MemoryProtection,
    pub base_address: u64,
    pub bytes_processed: usize,
}

#[derive(Clone, Default, Debug)]
pub struct AllocateMemoryResponse {
    pub mdl: MdlObject,
    pub bytes_allocated: u32,
}

#[derive(Clone, Default, Debug)]
pub struct MapMemoryResponse {
    pub mapped_address: u64,
}

impl VmcallResponse for MapMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            mapped_address: raw.arg1,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::MapMemory),
            arg1: self.mapped_address,

            ..Default::default()
        }
    }
}

impl VmcallResponse for AllocateMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            mdl: raw.arg1,
            bytes_allocated: raw.arg2 as _,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::AllocateMemory),
            arg1: self.mdl,
            arg2: self.bytes_allocated as _,

            ..Default::default()
        }
    }
}

impl VmcallResponse for ProtectProcessMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            old_protection: MemoryProtection::from_bits(raw.arg1 as _).unwrap(),
            base_address: raw.arg2 as _,
            bytes_processed: raw.arg3 as _,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::ProtectProcessMemory),
            arg1: self.old_protection.bits() as _,
            arg2: self.base_address as _,
            arg3: self.bytes_processed as _,

            ..Default::default()
        }
    }
}

impl VmcallResponse for RWProcessMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            bytes_processed: raw.arg1 as _,
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::ProcessVMOperation),
            arg1: self.bytes_processed as _,

            ..Default::default()
        }
    }
}
