use alloc::boxed::Box;
use core::any::{Any, TypeId};
use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::vmcall;
use crate::services::async_service::{AsyncPromise, GLOBAL_ASYNC_NOTIFY_HANDLER};

pub mod async_help;
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
    type Response: VmcallResponse + Any + Send + Sync;
    fn into_raw(self) -> HypervisorRequest;
    fn from_raw(call: HypervisorCall, args: (u64, u64, u64)) -> Self;
}

pub trait Vmcall<T: VmcallRequest> {
    fn send(self) -> Result<T::Response, HypervisorError>;
    fn send_async(self) -> u16;
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        T::Response::from_raw(vmcall(self.into_raw()))
    }

    fn send_async(self) -> u16 {
        let mut raw = self.into_raw();

        raw.call.set_is_async(true);
        let mut lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let mut promise = lock.new_promise();

        unsafe { GLOBAL_ASYNC_NOTIFY_HANDLER.force_unlock() }

        raw.call.set_async_cookie(promise.cookie >> 5);

        vmcall(raw);

        promise.cookie
    }
}
