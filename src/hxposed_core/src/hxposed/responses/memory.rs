use crate::hxposed::call::HypervisorResult;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::hxposed::RmdObject;

#[derive(Clone)]
pub struct PageAttributeResponse {
    pub type_bits: u64,
}

#[derive(Clone)]
pub struct TranslateAddressResponse {
    pub physical_addr: u64
}

#[derive(Clone)]
pub struct AllocateMemoryResponse {
    pub rmd: RmdObject
}

#[derive(Clone)]
pub struct DescribeMemoryResponse {
    pub rmd: RmdObject
}

impl VmcallResponse for DescribeMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            rmd: raw.arg1
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: self.rmd,
            ..Default::default()
        }
    }
}

impl VmcallResponse for TranslateAddressResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            physical_addr: raw.arg1
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: self.physical_addr,
                ..Default::default()
        }
    }
}

impl VmcallResponse for AllocateMemoryResponse {
    fn from_raw(raw: HypervisorResponse) -> Self {
        Self {
            rmd: raw.arg1
        }
    }

    fn into_raw(self) -> HypervisorResponse {
        HypervisorResponse {
            result: HypervisorResult::ok(),
            arg1: self.rmd,
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
            result: HypervisorResult::ok(),
            arg1: self.type_bits,
            arg2: 0,
            arg3: 0
        }
    }
}