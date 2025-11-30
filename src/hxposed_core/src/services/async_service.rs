use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::InternalErrorCode;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::hxposed::requests::HypervisorRequest;
use crate::intern::instructions::vmcall;
#[cfg(feature = "usermode")]
use crate::intern::win::SetEvent;
#[cfg(feature = "usermode")]
use crate::intern::win::{CloseHandle, CreateEventA, WaitForSingleObject};

///
/// # Global Async Notify Handler
///
/// The global handler anyone can access.
///
#[cfg(feature = "usermode")]
pub static GLOBAL_ASYNC_NOTIFY_HANDLER: HxPosedAsyncService = HxPosedAsyncService::new();

#[derive(Debug, Default)]
pub struct HxPosedAsyncService {}

#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    // memory is manually managed, unfortunately
    pub result_values: AtomicPtr<u64>,
}

impl Clone for AsyncInfo {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
            result_values: AtomicPtr::new(self.result_values.load(Ordering::Relaxed)),
        }
    }
}

#[cfg(feature = "usermode")]
#[derive(Clone, Debug, Default)]
pub struct AsyncPromise<T> {
    pub request: HypervisorRequest,
    pub completed: bool,
    phantom: PhantomData<T>,
}
#[cfg(feature = "usermode")]
impl<T> AsyncPromise<T>
where
    T: VmcallResponse,
{
    pub fn send_async(&self) {
        let response = vmcall(self.request.clone());
        if response.result.is_error() {
            unsafe {
                let ptr = self
                    .request
                    .async_info
                    .result_values
                    .load(Ordering::Relaxed);
                ptr.write(response.result.into_bits() as _);
                ptr.offset(1).write(response.arg1);
                ptr.offset(2).write(response.arg2);
                ptr.offset(3).write(response.arg3);
                SetEvent(self.request.async_info.handle);
            }
        }
    }

    ///
    /// # Wait<T>
    ///
    /// Waits for the async promise to be completed.
    /// T must be a type of [`VmcallResponse`], which the request was sent for.
    ///
    /// ## Arguments
    /// * `cookie` - [`AsyncPromise`] to wait for.
    ///
    /// ## Return
    /// - [R`esult`] with the [`VmcallResponse`] on [`T`]
    pub fn wait(self) -> Result<T, HypervisorError> {
        unsafe { WaitForSingleObject(self.request.async_info.handle, u32::MAX) };

        let ptr = self
            .request
            .async_info
            .result_values
            .load(Ordering::Relaxed);
        let result = unsafe {
            T::from_raw(HypervisorResponse {
                result: HypervisorResult::from_bits(ptr.read() as _),
                arg1: ptr.offset(1).read() as _,
                arg2: ptr.offset(1).read() as _,
                arg3: ptr.offset(1).read() as _,
            })
        };

        unsafe {
            CloseHandle(self.request.async_info.handle);
        }

        result
    }

    ///
    /// # Wait Timespan<T>
    ///
    /// Waits for the async promise to be completed with number of milliseconds of maximum.
    /// T must be a type of [`VmcallResponse`], which the request was sent for.
    ///
    /// ## Arguments
    /// * `cookie `- [`AsyncPromise`] to wait for.
    /// * `s `- Maximum milliseconds to wait.
    ///
    /// ## Return
    /// * [`InternalErrorCode::NotFound`] - Timed out.
    /// * [`T`] - Hypervisor responded.
    pub fn wait_timespan(self, ms: u32) -> Result<T, HypervisorError> {
        let result = match unsafe { WaitForSingleObject(self.request.async_info.handle, ms) } {
            0 => unsafe {
                let ptr = self
                    .request
                    .async_info
                    .result_values
                    .load(Ordering::Relaxed);
                T::from_raw(HypervisorResponse {
                    result: HypervisorResult::from_bits(ptr.read() as _),
                    arg1: ptr.offset(1).read() as _,
                    arg2: ptr.offset(2).read() as _,
                    arg3: ptr.offset(3).read() as _,
                })
            },
            0x102 => Err(HypervisorError::not_found()),

            _ => Err(HypervisorError::not_found()),
        };

        unsafe {
            CloseHandle(self.request.async_info.handle);
        }

        result
    }
}
#[cfg(feature = "usermode")]
impl HxPosedAsyncService {
    pub fn new_promise<T>(mut request: HypervisorRequest) -> Box<AsyncPromise<T>>
    where
        T: VmcallResponse,
    {
        request.call.set_is_async(true);
        request.async_info.handle = unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) };
        request.async_info.result_values = AtomicPtr::new(unsafe {
            Box::leak(Box::<[u64; 4]>::new_uninit().assume_init()) as *mut _ as *mut u64
        });

        Box::new(AsyncPromise::<T> {
            request,
            completed: false,
            phantom: PhantomData,
        })
    }

    pub const fn new() -> Self {
        Self {}
    }
}
