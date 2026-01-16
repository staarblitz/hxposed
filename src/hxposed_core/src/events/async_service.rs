use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorResult;
use crate::hxposed::requests::{HypervisorRequest, VmcallRequest};
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::intern::instructions::vmcall;
#[cfg(feature = "usermode")]
use crate::intern::win::{CloseHandle, CreateEventA, CreateThread, SetEvent, WaitForSingleObject};
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::marker::PhantomData;
use core::pin::Pin;
use core::ptr::null_mut;
use core::sync::atomic::{Ordering};
use core::task::{Context, Poll};
use spin::Mutex;
use crate::events::{AsyncInfo, AtomicPromiseState, PromiseState, WakerCell};

#[derive(Debug)]
pub struct AsyncPromise<RQ, RS> {
    pub async_info: Arc<AsyncInfo>,
    phantom: PhantomData<RS>,
    request: Option<RQ>,
    raw_request: Option<HypervisorRequest>,
}
impl<T, X> Unpin for AsyncPromise<T, X> {}

impl<RQ, RS> Drop for AsyncPromise<RQ, RS> {
    fn drop(&mut self) {
        self.async_info.state.store(PromiseState::CancelPending, Ordering::Relaxed);
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
        let me = Pin::get_mut(self);
        
        match me.async_info.state.load(Ordering::Relaxed) {
            PromiseState::None => {
                me.async_info.state.store(PromiseState::Waiting, Ordering::Relaxed);
                me.async_info.waker.register(cx.waker());

                let shared_ptr = Arc::into_raw(me.async_info.clone());

                match unsafe {
                    CreateThread(
                        null_mut(),
                        0,
                        Self::hv_wait_worker,
                        shared_ptr as _,
                        0,
                        null_mut(),
                    )
                } {
                    0 => {
                        // reclaim mem
                        /*unsafe{
                            drop(Arc::from_raw(shared_ptr));
                        }*/
                        panic!("CreateThread returned an error")
                    }
                    handle => unsafe {
                        CloseHandle(handle)
                    },
                };
                Poll::Pending
            }
            PromiseState::Waiting => {
                me.async_info.waker.register(cx.waker());
                Poll::Pending
            },
            PromiseState::CancelPending => Poll::Ready(Err(HypervisorError::async_cancel())),
            PromiseState::TimedOut => Poll::Ready(Err(HypervisorError::async_time_out())),
            PromiseState::Completed => {
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
        let info = Arc::from_raw(param as *const AsyncInfo);

        let complete = |state: PromiseState| {
            info.state.store(state, Ordering::Relaxed);
            info.waker.wake_and_clear();
        };

        match WaitForSingleObject(info.handle, 2000) {
            0x00000102 /* WAIT_TIMEOUT*/ => {
                complete(PromiseState::TimedOut)
            }
            0 => {}
            err => {
                panic!("WaitForSingleObject failed: {:x}", err);
            }
        }

        complete(PromiseState::Completed);

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

        let response = vmcall(&mut raw_request, Some(self.async_info.clone()));

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
            phantom: PhantomData,
            async_info: Arc::new(AsyncInfo {
                handle: unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) },
                result_values: Mutex::new(Box::new([0; 4])),
                waker: WakerCell::new(),
                state: AtomicPromiseState::new(PromiseState::None),
            }),
        })
    }
}
