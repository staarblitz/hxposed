use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::error::ErrorCode;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};
use spin::Mutex;

#[cfg(feature = "usermode")]
use crate::intern::win::{CloseHandle, CreateEventA, WaitForSingleObject};

///
/// # Global Async Notify Handler
///
/// The global handler anyone can access.
///
#[cfg(feature = "usermode")]
pub static GLOBAL_ASYNC_NOTIFY_HANDLER: Mutex<HxPosedAsyncService> =
    Mutex::new(HxPosedAsyncService::new());

#[derive(Debug, Default)]
pub struct HxPosedAsyncService {}

#[derive(Default)]
pub struct AsyncInfo {
    pub handle: u64,
    pub result_values: AtomicPtr<[u64; 4]>,
}

impl Clone for AsyncInfo {
    fn clone(&self) -> Self {
        Self{
            handle: self.handle,
            result_values: AtomicPtr::new(self.result_values.load(Ordering::Relaxed)),
        }
    }
}
#[cfg(feature = "usermode")]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct AsyncPromise<T> {
    pub event: u64,
    pub completed: bool,
    pub result: HypervisorResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    phantom: PhantomData<T>,
}
#[cfg(feature = "usermode")]
impl<T> AsyncPromise<T>
where
    T: VmcallResponse,
{
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
        unsafe { WaitForSingleObject(self.event, u32::MAX) };

        let result = T::from_raw(HypervisorResponse {
            result: self.result,
            arg1: self.arg1,
            arg2: self.arg2,
            arg3: self.arg3,
        });

        unsafe {
            CloseHandle(self.event);
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
    /// * [`ErrorCode::NotFound`] - Timed out.
    /// * [`T`] - Hypervisor responded.
    pub fn wait_timespan(self, ms: u32) -> Result<T, HypervisorError> {
        let result = match unsafe { WaitForSingleObject(self.event, ms) } {
            0 => T::from_raw(HypervisorResponse {
                result: self.result,
                arg1: self.arg1,
                arg2: self.arg2,
                arg3: self.arg3,
            }),
            0x102 => Err(HypervisorError::not_found()),

            _ => Err(HypervisorError::not_found()),
        };

        unsafe {
            CloseHandle(self.event);
        }

        result
    }
}
#[cfg(feature = "usermode")]
impl HxPosedAsyncService {
    pub fn new_promise<T>(&mut self) -> Box<AsyncPromise<T>>
    where
        T: VmcallResponse,
    {
        Box::new(AsyncPromise::<T> {
            event: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
            arg1: 0,
            arg2: 0,
            arg3: 0,
            completed: false,
            result: HypervisorResult::default(),
            phantom: PhantomData,
        })
    }

    pub const fn new() -> Self {
        Self {}
    }
}
