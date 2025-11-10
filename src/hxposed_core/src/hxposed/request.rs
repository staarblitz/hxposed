use crate::hxposed::call::HypervisorCall;
use crate::hxposed::response::VmcallResponse;

#[derive(Copy, Clone, Default, Debug)]
pub struct HypervisorRequest {
    pub(crate) call: HypervisorCall,
    pub(crate) arg1: u64,
    pub(crate) arg2: u64,
    pub(crate) arg3: u64,
}

pub trait VmcallRequest {
    type Response: VmcallResponse;

    fn into_raw(self) -> HypervisorRequest;
    fn send(self) -> Self::Response;
}
