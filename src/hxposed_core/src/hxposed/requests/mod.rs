#![allow(unused_imports)]

use crate::error::HxError;
use crate::hxposed::call::HxCall;
use crate::hxposed::responses::SyscallResponse;
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
pub mod io;
pub mod handle;

#[derive(Clone, Default, Debug)]
pub struct HxRequest {
    pub call: HxCall,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,

    // pub extended_args: [MaybeUninit<u128>; 4]
    pub extended_arg1: u128,
    pub extended_arg2: u128,
    pub extended_arg3: u128,
    pub extended_arg4: u128,
}
pub trait SyscallRequest {
    type Response: SyscallResponse + Any + Send + Sync + Clone;
    fn into_raw(self) -> HxRequest;
    fn from_raw(request: &HxRequest) -> Self;
}

pub trait Syscall<T: SyscallRequest> {
    fn send(self) -> Result<T::Response, HxError>;
}

impl<T> Syscall<T> for T
where
    T: SyscallRequest,
{
    fn send(self) -> Result<T::Response, HxError> {
        let response = vmcall(&mut self.into_raw());
        if response.result.error_code != 0 {
            Err(HxError::from_response(&response))
        } else {
            Ok(T::Response::from_raw(response))
        }
    }
}
