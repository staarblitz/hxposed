use crate::error::HypervisorError;
use crate::hxposed::call::{HypervisorCall, HypervisorResult};
use crate::hxposed::error::ErrorCode;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::async_help::{AddAsyncHandlerRequest, RemoveAsyncHandlerRequest};
use crate::hxposed::requests::process::KillProcessRequest;
use crate::hxposed::requests::{Vmcall, VmcallRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::{HypervisorResponse, VmcallResponse};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::arch::{asm, naked_asm};
use core::cell::UnsafeCell;
use core::ops::Deref;
use core::sync::atomic::AtomicPtr;
use spin::Mutex;

///
/// # Global Async Notify Handler
///
/// The global handler anyone can access.
///
pub static GLOBAL_ASYNC_NOTIFY_HANDLER: Mutex<HxPosedAsyncService> =
    Mutex::new(HxPosedAsyncService::new());

#[derive(Debug, Default)]
pub struct HxPosedAsyncService {
    promises: Vec<Box<AsyncPromise>>,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct AsyncPromise {
    pub cookie: u16,
    pub completed: bool,
    pub result: HypervisorResult,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
}

impl AsyncPromise {

    ///
    /// # Spin Wait<T>
    ///
    /// Waits for the async promise to be completed.
    /// T must be a type of [VmcallResponse], which the request was sent for.
    ///
    /// ## Arguments
    /// cookie - AsyncPromise to wait for.
    ///
    /// ## Warning
    /// The lock is hold forever if hypervisor doesn't respond (which should NEVER happen).
    ///
    /// ## Return
    /// [Result] with the [VmcallResponse] on [T]
    pub fn spin_wait<T>(cookie: u16) -> Result<T, HypervisorError>
    where
        T: VmcallResponse,
    {

        let lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let promise = match lock.promises.iter().find(|p| p.cookie == cookie) {
            Some(p) => p,
            None => return Err(HypervisorError::not_found())
        };

        loop {
            if promise.completed {
                return T::from_raw(HypervisorResponse {
                    result: promise.result,
                    arg1: promise.arg1,
                    arg2: promise.arg2,
                    arg3: promise.arg3,
                });
            }
        }
    }

    ///
    /// # Spin Wait Tries<T>
    ///
    /// Waits for the async promise to be completed with number of iterations (**not seconds**) of maximum.
    /// T must be a type of [VmcallResponse], which the request was sent for.
    ///
    /// ## Arguments
    /// cookie - AsyncPromise to wait for.
    /// s - Maximum number of iterations
    ///
    /// ## Warning
    /// The lock is hold forever if hypervisor doesn't respond (which should NEVER happen).
    ///
    /// ## Return
    /// [Result] with the [VmcallResponse] on [T].
    /// Returns [ErrorCode::NotFound] if s is reached.
    pub fn spin_wait_tries<T>(cookie: u16, s: u32) -> Result<T, HypervisorError>
    where
        T: VmcallResponse,
    {
        let lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let promise = match lock.promises.iter().find(|p| p.cookie == cookie) {
            Some(p) => p,
            None => return Err(HypervisorError::not_found())
        };

        let mut iter = s;

        loop {
            if promise.completed {
                return T::from_raw(HypervisorResponse {
                    result: promise.result,
                    arg1: promise.arg1,
                    arg2: promise.arg2,
                    arg3: promise.arg3,
                });
            } else if iter > s {
                return Err(HypervisorError::not_found());
            }

            iter += 1;
        }
    }
}

/// MUST BE USED AFTER AUTHORIZATION!
#[derive(Debug, Eq, PartialEq)]
pub struct AsyncNotifyHandler {
    pub handler: AsyncNotifyFn,
}

pub type AsyncNotifyFn = fn(function: ServiceFunction, args: (u64, u64, u64));

impl HxPosedAsyncService {
    #[unsafe(naked)]
    unsafe extern "C" fn async_event() {
        naked_asm!(
            "mov rsi, rcx",
            "mov r8, rdx",
            "mov r9, r8",
            "mov r10, r9",
            "call safe_async_event"
        )
    }

    #[unsafe(no_mangle)]
    extern "C" fn safe_async_event(
        hypervisor_result: HypervisorResult,
        arg1: u64,
        arg2: u64,
        arg3: u64,
    ) {
        let cookie = hypervisor_result.cookie();

        let mut lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let result = match lock.promises.iter_mut().find(|x| x.cookie == cookie) {
            Some(x) => x,
            None => return,
        };

        let bo = match hypervisor_result.func() {
            ServiceFunction::KillProcess => EmptyResponse {},
            _ => return,
        };

        result.result = hypervisor_result;
        result.arg1 = arg1;
        result.arg2 = arg2;
        result.arg3 = arg3;

        result.completed = true; // setting it last to avoid race condition
    }
}

impl HxPosedAsyncService {
    pub fn new_promise(&mut self) -> &Box<AsyncPromise> {
        let mut promise = AsyncPromise::default();
        let mut rnd = 0;
        unsafe {
            // not using a crate just to get random numbers working lol
            asm!("rdrand {0}", inout(reg) rnd => rnd);
        }

        promise.cookie = rnd;

        let promise = Box::new(promise);

        self.promises.push(promise);
        self.promises.last().unwrap()
    }

    pub fn init(&mut self) -> Result<EmptyResponse, HypervisorError> {
        AddAsyncHandlerRequest {
            addr: Self::async_event as *const u64 as u64,
        }
        .send()
    }

    pub const fn new() -> Self {
        Self {
            promises: Vec::new(),
        }
    }
}
