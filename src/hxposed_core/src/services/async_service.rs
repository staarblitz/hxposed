use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::HypervisorRequest;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::intern::instructions::vmcall;
#[cfg(feature = "usermode")]
use crate::intern::win::{CloseHandle, CreateEventA, CreateThread, SetEvent, WaitForSingleObject};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use core::task::{Context, Poll, Waker};

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
    pub result_values: *mut [u64; 4],
}
#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    // memory is manually managed, unfortunately
    pub result_values: Box<[u64; 4]>,
}

#[cfg(feature = "usermode")]
#[derive(Debug, Default)]
pub struct WakerCell {
    ptr: AtomicPtr<Waker>,
}

#[cfg(feature = "usermode")]
impl WakerCell {
    pub const fn new() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }

    pub fn register(&self, w: &Waker) {
        // allocate Box<Waker> on heap
        let boxed = Box::into_raw(Box::new(w.clone()));
        // swap old pointer out; if existing, drop it
        let prev = self.ptr.swap(boxed, Ordering::AcqRel);
        if !prev.is_null() {
            // safety: prev was a Box<Waker> allocated above
            unsafe { drop(Box::from_raw(prev)) };
        }
    }

    pub fn take(&self) -> Option<Waker> {
        let p = self.ptr.swap(null_mut(), Ordering::AcqRel);
        if p.is_null() {
            None
        } else {
            // safety: p is Box<Waker>
            let boxed = unsafe { Box::from_raw(p) };
            Some(*boxed)
        }
    }

    pub fn wake_and_clear(&self) {
        if let Some(w) = self.take() {
            w.wake();
        }
    }
}

#[cfg(feature = "usermode")]
impl Drop for WakerCell {
    fn drop(&mut self) {
        let p = self.ptr.load(Ordering::Acquire);
        if !p.is_null() {
            unsafe { drop(Box::from_raw(p)) };
        }
    }
}

#[cfg(feature = "usermode")]
#[derive(Debug, Default)]
pub struct AsyncPromise<T> {
    pub request: HypervisorRequest,
    pub async_info: AsyncInfo,
    pub completed: AtomicBool,
    waker: WakerCell,
    phantom: PhantomData<T>,
}
#[cfg(feature = "usermode")]
impl<T> Unpin for AsyncPromise<T> {}

#[cfg(feature = "usermode")]
impl<T> Future for AsyncPromise<T>
where
    T: VmcallResponse,
{
    type Output = Result<T, HypervisorError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = unsafe { self.get_unchecked_mut() };

        if me.completed.load(Ordering::Acquire) {
            unsafe {
                CloseHandle(me.async_info.handle);
            }
            return Poll::Ready(me.get_result());
        }

        me.waker.register(cx.waker());
        // double check to avoid races
        if me.completed.load(Ordering::Acquire) {
            unsafe {
                CloseHandle(me.async_info.handle);
            }
            return Poll::Ready(me.get_result());
        }

        match unsafe {
            CreateThread(
                null_mut(),
                0,
                Self::hv_wait_worker,
                Pin::new_unchecked(me).get_unchecked_mut() as *mut _ as _,
                0,
                null_mut(),
            )
        } {
            0 => panic!("CreateThread returned an error"),
            handle => unsafe { CloseHandle(handle) },
        }

        Poll::Pending
    }
}

#[cfg(feature = "usermode")]
impl<T> AsyncPromise<T>
where
    T: VmcallResponse,
{
    unsafe extern "C" fn hv_wait_worker(param: *mut u64) -> u32 {
        let me = &mut *(param as *mut AsyncPromise<T>);
        WaitForSingleObject(me.async_info.handle, u32::MAX);

        me.completed.store(true, Ordering::Relaxed);
        if let Some(waker) = me.waker.take() {
            waker.wake();
        }

        0
    }

    fn get_result(&mut self) -> Result<T, HypervisorError> {
        let ptr = self.async_info.result_values.as_mut_ptr();

        unsafe {
            T::from_raw(HypervisorResponse {
                result: HypervisorResult::from_bits(ptr.read() as _),
                arg1: ptr.offset(1).read() as _,
                arg2: ptr.offset(2).read() as _,
                arg3: ptr.offset(3).read() as _,
            })
        }
    }

    pub fn send_async(&mut self) {
        let response = vmcall(self.request.clone(), Some(&mut self.async_info));
        if response.result.is_error() {
            unsafe {
                let ptr = self.async_info.result_values.as_mut_ptr();

                ptr.write(response.result.into_bits() as _);
                ptr.offset(1).write(response.arg1);
                ptr.offset(2).write(response.arg2);
                ptr.offset(3).write(response.arg3);
                SetEvent(self.async_info.handle);
            }
        }
    }
}
#[cfg(feature = "usermode")]
impl HxPosedAsyncService {
    pub fn new_promise<T>(mut request: HypervisorRequest) -> Pin<Box<AsyncPromise<T>>>
    where
        T: VmcallResponse,
    {
        request.call.set_is_async(true);

        Box::pin(AsyncPromise::<T> {
            request,
            completed: AtomicBool::new(false),
            waker: WakerCell::new(),
            phantom: PhantomData,
            async_info: AsyncInfo {
                handle: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
                result_values: Box::new([0; 4]),
            },
        })
    }

    pub const fn new() -> Self {
        Self {}
    }
}
