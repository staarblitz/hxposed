use crate::hxposed::call::HypervisorResult;

pub mod status;
pub mod process;
pub mod auth;

#[derive(Copy, Clone, Default, Debug)]
pub struct HypervisorResponse {
    pub result: HypervisorResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

pub trait VmcallResponse: Sized {
    fn from_raw(raw: HypervisorResponse) -> Self;
    fn into_raw(self) -> HypervisorResponse;
}
