use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::InternalErrorCode;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
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
pub struct UnsafeAsyncInfo {
    pub handle: u64,
    pub result_values: *mut [u64;4]
}
#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    // memory is manually managed, unfortunately
    pub result_values: Box<[u64;4]>,
}

#[cfg(feature = "usermode")]
#[derive(Debug, Default)]
pub struct AsyncPromise<T> {
    pub request: HypervisorRequest,
    pub async_info: AsyncInfo,
    pub completed: bool,
    phantom: PhantomData<T>,
}
#[cfg(feature = "usermode")]
impl<T> AsyncPromise<T>
where
    T: VmcallResponse,
{
    pub fn send_async(&mut self) {
        let response = vmcall(self.request.clone(), Some(&mut self.async_info));
        if response.result.is_error() {
            unsafe {
                let ptr = self

                    .async_info
                    .result_values.as_mut_ptr();

                ptr.write(response.result.into_bits() as _);
                ptr.offset(1).write(response.arg1);
                ptr.offset(2).write(response.arg2);
                ptr.offset(3).write(response.arg3);
                SetEvent(self.async_info.handle);
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
    pub fn wait(mut self) -> Result<T, HypervisorError> {
        unsafe { WaitForSingleObject(self.async_info.handle, u32::MAX) };

        let ptr = self
            .async_info
            .result_values.as_mut_ptr();

        let result = unsafe {
            T::from_raw(HypervisorResponse {
                result: HypervisorResult::from_bits(ptr.read() as _),
                arg1: ptr.offset(1).read() as _,
                arg2: ptr.offset(2).read() as _,
                arg3: ptr.offset(3).read() as _,
            })
        };

        unsafe {
            CloseHandle(self.async_info.handle);
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
    pub fn wait_timespan(mut self, ms: u32) -> Result<T, HypervisorError> {
        let result = match unsafe { WaitForSingleObject(self.async_info.handle, ms) } {
            0 => unsafe {
                let ptr = self

                    .async_info
                    .result_values.as_mut_ptr();

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
            CloseHandle(self.async_info.handle);
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

        Box::new(AsyncPromise::<T> {
            request,
            completed: false,
            phantom: PhantomData,
            async_info:  AsyncInfo {
                handle: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
                result_values: Box::new([0;4])
            }
        })
    }

    pub const fn new() -> Self {
        Self {}
    }
}
