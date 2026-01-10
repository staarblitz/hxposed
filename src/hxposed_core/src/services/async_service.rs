use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::VmcallRequest;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::intern::instructions::vmcall;
#[cfg(feature = "usermode")]
use crate::intern::win::{CloseHandle, CreateEventA, CreateThread, SetEvent, WaitForSingleObject};
use alloc::boxed::Box;
use atomic_enum::atomic_enum;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::task::{Context, Poll, Waker};
use spin::Mutex;

///
/// # Global Async Notify Handler
///
/// The global handler anyone can access.
///

#[derive(Debug, Default)]
pub struct HxPosedAsyncService {}

#[derive(Default, Debug)]
pub struct UnsafeAsyncInfo {
    pub handle: u64,
    pub result_values: *mut [u64; 4],
}

impl UnsafeAsyncInfo {
    pub fn is_present(&self) -> bool {
        self.handle != 0
    }
}

#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    // memory is manually managed, unfortunately
    pub result_values: Mutex<Box<[u64; 4]>>,
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
        let boxed = Box::into_raw(Box::new(w.clone()));
        let prev = self.ptr.swap(boxed, Ordering::AcqRel);
        if !prev.is_null() {
            // SAFETY: prev was a Box<Waker> allocated above
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
#[atomic_enum]
pub enum PromiseState {
    None,
    Waiting,
    Completed,
}

#[cfg(feature = "usermode")]
#[derive(Debug)]
pub struct AsyncPromise<T, X> {
    pub async_info: AsyncInfo,
    pub state: AtomicPromiseState,
    waker: WakerCell,
    phantom: PhantomData<X>,
    request: Option<T>,
    raw_request: Option<*mut T>,
}
#[cfg(feature = "usermode")]
impl<T, X> Unpin for AsyncPromise<T, X> {}

#[cfg(feature = "usermode")]
impl<T, X> Future for AsyncPromise<T, X>
where
    X: VmcallResponse,
    T: VmcallRequest,
{
    type Output = Result<X, HypervisorError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = unsafe { self.get_unchecked_mut() };

        match me.state.load(Ordering::Acquire) {
            PromiseState::None => {
                me.waker.register(cx.waker());
                let pinned_me = unsafe{Pin::new_unchecked(me).get_unchecked_mut()};
                match unsafe {
                    CreateThread(
                        null_mut(),
                        0,
                        Self::hv_wait_worker,
                        pinned_me as *mut _ as _,
                        0,
                        null_mut(),
                    )
                } {
                    0 => panic!("CreateThread returned an error"),
                    handle => unsafe {
                        CloseHandle(handle)
                    },
                };

                Poll::Pending
            }
            PromiseState::Waiting => Poll::Pending,
            PromiseState::Completed => {
                // unsafe {
                //     asm!("int 0x3")
                // }
                Poll::Ready(me.get_result())
            }
        }
    }
}

#[cfg(feature = "usermode")]
impl<T, X> AsyncPromise<T, X>
where
    X: VmcallResponse,
    T: VmcallRequest,
{
    unsafe extern "C" fn hv_wait_worker(param: *mut u64) -> u32 {
        let me = &mut *(param as *mut AsyncPromise<T, X>);
        WaitForSingleObject(me.async_info.handle, u32::MAX);

        me.state.store(PromiseState::Completed, Ordering::Release);
        if let Some(waker) = me.waker.take() {
            waker.wake();
        }

        0
    }

    fn get_result(&mut self) -> Result<X, HypervisorError> {
        let ptr = self.async_info.result_values.lock().as_mut_ptr();

        let result = unsafe {
            X::from_raw(HypervisorResponse {
                result: HypervisorResult::from_bits(ptr.read() as _),
                arg1: ptr.offset(1).read() as _,
                arg2: ptr.offset(2).read() as _,
                arg3: ptr.offset(3).read() as _,
            })
        };

        // we are good to go, now we can free the memory we leaked.

        if let Some(ptr) = self.raw_request {
            unsafe { drop(Box::from_raw(ptr)) }
        }

        result
    }


    pub fn send_async(&mut self) {
        let request = self
            .request
            .take()
            .expect("send_async called more than once");

        let raw_request = request.into_raw();

        // save it for later
        self.raw_request = Some(raw_request as *mut T);

        let response = vmcall(raw_request, Some(&mut self.async_info));
        if response.result.is_error() {
            unsafe {
                let ptr = self.async_info.result_values.lock().as_mut_ptr();

                ptr.write(response.result.into_bits() as _);
                ptr.offset(1).write(response.arg1);
                ptr.offset(2).write(response.arg2);
                ptr.offset(3).write(response.arg3);
                SetEvent(self.async_info.handle);
            }
        }
    }

    pub fn new_promise(request: T) -> Pin<Box<AsyncPromise<T, X>>> {
        Box::pin(AsyncPromise::<T, X> {
            request: Some(request),
            raw_request: None,
            state: AtomicPromiseState::new(PromiseState::None),
            waker: WakerCell::new(),
            phantom: PhantomData,
            async_info: AsyncInfo {
                handle: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
                result_values: Mutex::new(Box::new([0; 4])),
            },
        })
    }
}
