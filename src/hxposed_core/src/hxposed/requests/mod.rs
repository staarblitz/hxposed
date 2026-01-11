#![allow(unused_imports)]

use crate::error::HypervisorError;
#[cfg(feature = "usermode")]
use crate::events::async_service::AsyncPromise;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::vmcall;
use alloc::boxed::Box;
use core::any::Any;
use core::pin::Pin;

pub mod auth;
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
    #[cfg(feature = "usermode")]
    fn send_async(self) -> Pin<Box<AsyncPromise<T, T::Response>>>;
    #[cfg(feature = "usermode")]
    fn get_promise(self) -> Pin<Box<AsyncPromise<T, T::Response>>>;
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        let response = vmcall(self.into_raw(), None);
        if response.result.is_error() {
            Err(HypervisorError::from_response(response))
        } else {
            Ok(T::Response::from_raw(response))
        }
    }

    #[cfg(feature = "usermode")]
    fn send_async(self) -> Pin<Box<AsyncPromise<T, T::Response>>> {
        let mut promise = self.get_promise();

        promise.send_async();

        promise
    }

    #[cfg(feature = "usermode")]
    fn get_promise(self) -> Pin<Box<AsyncPromise<T, T::Response>>> {
        AsyncPromise::<T, T::Response>::new_promise(self)
    }
}
