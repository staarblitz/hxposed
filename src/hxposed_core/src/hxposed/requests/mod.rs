use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::{vmcall, vmcall_typed};

pub mod status;
mod process;
pub mod auth;

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
}


trait Vmcall<T: VmcallRequest> {
    fn send(self) -> T::Response;
}

impl<T> Vmcall<T> for T
where T: VmcallRequest {
    fn send(self) -> T::Response {
        T::Response::from_raw(vmcall(self.into_raw()))
    }
}