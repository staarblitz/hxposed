#![allow(unused_imports)]

use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::vmcall;
use alloc::boxed::Box;
use core::any::Any;
use core::pin::Pin;

pub mod memory;
pub mod notify;
pub mod process;
pub mod security;
pub mod status;
pub mod thread;

#[derive(Clone, Default, Debug)]
pub struct HypervisorRequest {
    pub call: HypervisorCall,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,

    pub extended_arg1: u128,
    pub extended_arg2: u128,
    pub extended_arg3: u128,
    pub extended_arg4: u128,
}
pub trait VmcallRequest {
    type Response: VmcallResponse + Any + Send + Sync + Clone;
    fn into_raw(self) -> HypervisorRequest;
    fn from_raw(request: &HypervisorRequest) -> Self;
}

pub trait Vmcall<T: VmcallRequest> {
    fn send(self) -> Result<T::Response, HypervisorError>;
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        let response = vmcall(&mut self.into_raw());
        if response.result.is_error() {
            Err(HypervisorError::from_response(response))
        } else {
            Ok(T::Response::from_raw(response))
        }
    }
}
