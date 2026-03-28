use crate::hxposed::call::HxResult;
use crate::hxposed::responses::{HxResponse, SyscallResponse};
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

impl SyscallResponse for DescribeMemoryResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            rmd: raw.arg1
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.rmd,
            ..Default::default()
        }
    }
}

impl SyscallResponse for TranslateAddressResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            physical_addr: raw.arg1
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.physical_addr,
                ..Default::default()
        }
    }
}

impl SyscallResponse for AllocateMemoryResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            rmd: raw.arg1
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.rmd,
            ..Default::default()
        }
    }
}

impl SyscallResponse for PageAttributeResponse {
    fn from_raw(raw: HxResponse) -> Self {
        Self {
            type_bits: raw.arg1
        }
    }

    fn into_raw(self) -> HxResponse {
        HxResponse {
            result: HxResult::ok(),
            arg1: self.type_bits,
            arg2: 0,
            arg3: 0
        }
    }
}