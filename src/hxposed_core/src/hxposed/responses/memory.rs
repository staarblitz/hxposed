use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone)]
pub struct PageAttributeResponse {
    pub type_bits: u64,
}

#[derive(Clone)]
pub struct AllocateMemoryResponse {
    /// Also the RmdObject
    pub system_pa: u64
}

impl VmcallResponse for AllocateMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            system_pa: raw.arg1
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::AllocateMemory),
            arg1: self.system_pa,
            ..Default::default()
        }
    }
}

impl VmcallResponse for PageAttributeResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            type_bits: raw.arg1
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetSetPageAttribute),
            arg1: self.type_bits,
            arg2: 0,
            arg3: 0
        }
    }
}