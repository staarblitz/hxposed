use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
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
use crate::events::AsyncInfo;

#[derive(Debug, Default)]
pub struct WakerCell {
    ptr: AtomicPtr<Waker>,
}

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

impl Drop for WakerCell {
    fn drop(&mut self) {
        let p = self.ptr.load(Ordering::Acquire);
        if !p.is_null() {
            unsafe { drop(Box::from_raw(p)) };
        }
    }
}

#[atomic_enum]
pub enum PromiseState {
    None,
    Waiting,
    Completed,
}

#[derive(Debug)]
pub struct AsyncPromise<RQ, RS> {
    pub async_info: AsyncInfo,
    pub state: AtomicPromiseState,
    waker: WakerCell,
    phantom: PhantomData<RS>,
    request: Option<RQ>,
    raw_request: Option<HypervisorRequest>,
    cancellation_requested: bool
}
impl<T, X> Unpin for AsyncPromise<T, X> {}

impl<RQ, RS> Drop for AsyncPromise<RQ, RS> {
    fn drop(&mut self) {
        self.cancellation_requested = true;
        // wake up the waiting thread
        unsafe{
            SetEvent(self.async_info.handle);
        }
    }
}

impl<RQ, RS> Future for AsyncPromise<RQ, RS>
where
    RS: VmcallResponse,
    RQ: VmcallRequest,
{
    type Output = Result<RS, HypervisorError>;

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

impl<RQ, RS> AsyncPromise<RQ, RS>
where
    RS: VmcallResponse,
    RQ: VmcallRequest,
{
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe extern "C" fn hv_wait_worker(param: *mut u64) -> u32 {
        let me = &mut *(param as *mut AsyncPromise<RQ, RS>);
        match WaitForSingleObject(me.async_info.handle, 2000) {
            0x00000102 /* WAIT_TIMEOUT*/ => {
                panic!("Hypervisor failed to complete async task. This indicates a bug on kernel side.")
            }
            _ => {}
        }

        // check if cancellation requested, if so, do nothing.
        if me.cancellation_requested {
            return 0;
        }

        me.state.store(PromiseState::Completed, Ordering::Release);
        if let Some(waker) = me.waker.take() {
            waker.wake();
        }

        0
    }

    fn get_result(&mut self) -> Result<RS, HypervisorError> {
        let ptr = self.async_info.result_values.lock().as_mut_ptr();

        let response = unsafe {
          HypervisorResponse {
                result: HypervisorResult::from_bits(ptr.read() as _),
                arg1: ptr.offset(1).read() as _,
                arg2: ptr.offset(2).read() as _,
                arg3: ptr.offset(3).read() as _,
            }
        };

        if response.result.is_error(){
            Err(HypervisorError::from_response(response))
        } else {
            Ok(RS::from_raw(response))
        }
    }


    pub fn send_async(&mut self) {
        let request = self
            .request
            .take()
            .expect("send_async called more than once");

        let mut raw_request = request.into_raw();

        let response = vmcall(&mut raw_request, Some(&mut self.async_info));

        // save it for later
        self.raw_request = Some(raw_request);

        if response.result.is_error() {
            unsafe {
                let ptr = self.async_info.result_values.lock().as_mut_ptr();

                ptr.write(response.result.into_bits() as _);
                ptr.offset(1).write(response.arg1);
                ptr.offset(2).write(response.arg2);
                ptr.offset(3).write(response.arg3);

                // directly wakeup the thread with the failure values
                SetEvent(self.async_info.handle);
            }
        }
    }

    pub fn new_promise(request: RQ) -> Pin<Box<AsyncPromise<RQ, RS>>> {
        Box::pin(AsyncPromise::<RQ, RS> {
            request: Some(request),
            raw_request: None,
            state: AtomicPromiseState::new(PromiseState::None),
            waker: WakerCell::new(),
            phantom: PhantomData,
            cancellation_requested: false,
            async_info: AsyncInfo {
                handle: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
                result_values: Mutex::new(Box::new([0; 4])),
            },
        })
    }
}
