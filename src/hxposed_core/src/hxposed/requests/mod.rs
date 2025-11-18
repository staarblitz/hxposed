use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::{vmcall, vmcall_typed};

pub mod auth;
pub mod process;
pub mod status;

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
    fn from_raw(call: HypervisorCall, args: (u64,u64,u64)) -> Self;
}

pub trait Vmcall<T: VmcallRequest> {
    fn send(self) -> Result<T::Response, HypervisorError>;
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        T::Response::from_raw(vmcall(self.into_raw()))
    }
}
