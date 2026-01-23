use crate::hxposed::call::HypervisorResult;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::memory::PageAttributes;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};

#[derive(Clone)]
pub struct PageAttributeResponse {
    pub result: PageAttributes
}

#[derive(Clone)]
pub struct AllocateMemoryResponse {
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
            result: PageAttributes::from_raw_enum(raw.arg1, raw.arg2)
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        let args = self.result.into_raw_enum();
        HypervisorResponse {
            result: HypervisorResult::ok(ServiceFunction::GetSetPageAttribute),
            arg1: args.0,
            arg2: args.1,
            arg3: 0
        }
    }
}