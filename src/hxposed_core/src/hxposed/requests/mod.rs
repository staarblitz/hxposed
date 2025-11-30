use alloc::boxed::Box;
use core::any::{Any, TypeId};
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::responses::VmcallResponse;
use crate::intern::instructions::vmcall;
#[cfg(feature = "usermode")]
use crate::services::async_service::{AsyncPromise, GLOBAL_ASYNC_NOTIFY_HANDLER};

pub mod async_help;
pub mod auth;
pub mod process;
pub mod status;

#[derive(Default, Debug, Clone)]
pub struct HypervisorRequest {
    pub(crate) call: HypervisorCall,
    pub(crate) arg1: u64,
    pub(crate) arg2: u64,
    pub(crate) arg3: u64,
    pub(crate) async_handle: u64,
    pub(crate) result_data: Box<[u64;4]>
}

pub trait VmcallRequest {
    type Response: VmcallResponse + Any + Send + Sync + Clone;
    fn into_raw(self) -> HypervisorRequest;
    fn from_raw(call: HypervisorCall, args: (u64, u64, u64)) -> Self;
}

pub trait Vmcall<T: VmcallRequest> {
    fn send(self) -> Result<T::Response, HypervisorError>;
    #[cfg(feature = "usermode")]
    fn send_async(self) -> AsyncPromise<T::Response>;
}

impl<T> Vmcall<T> for T
where
    T: VmcallRequest,
{
    fn send(self) -> Result<T::Response, HypervisorError> {
        T::Response::from_raw(vmcall(self.into_raw()))
    }

    #[cfg(feature = "usermode")]
    fn send_async(self) -> AsyncPromise<T::Response> {
        let mut raw = self.into_raw();

        raw.call.set_is_async(true);
        let mut lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let mut promise = lock.new_promise::<T::Response>();

        unsafe { GLOBAL_ASYNC_NOTIFY_HANDLER.force_unlock() }

        raw.async_handle = promise.event;
        raw.result_memory = Box::new();

        vmcall(raw);

        (*promise).clone()
    }
}
