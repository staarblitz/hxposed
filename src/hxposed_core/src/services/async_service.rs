use crate::error::HypervisorError;
use crate::hxposed::call::HypervisorCall;
use crate::hxposed::error::ErrorCode;
use crate::hxposed::func::ServiceFunction;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::async_help::{AddAsyncHandlerRequest, RemoveAsyncHandlerRequest};
use crate::hxposed::responses::VmcallResponse;
use alloc::vec::Vec;
use core::arch::naked_asm;
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
    handlers: Vec<AsyncNotifyHandler>,
}

/// MUST BE USED AFTER AUTHORIZATION!
#[derive(Debug, Eq, PartialEq)]
pub struct AsyncNotifyHandler {
    pub handler: AsyncNotifyFn,
    pub cookie: u16,
    /// Filters callbacks. Only callbacks with service functions that are in this vector are triggered.
    pub filter: Vec<ServiceFunction>,
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
        hypervisor_call: HypervisorCall,
        arg1: u64,
        arg2: u64,
        arg3: u64,
    ) {
        let cookie = hypervisor_call.async_cookie();

        let lock = GLOBAL_ASYNC_NOTIFY_HANDLER.lock();
        let result = match lock.handlers.iter().find(|x| x.cookie == cookie) {
            Some(x) => x,
            None => return,
        };

        if !result.filter.iter().any(|x| *x == hypervisor_call.func()) {
            return;
        }

        unsafe{
            GLOBAL_ASYNC_NOTIFY_HANDLER.force_unlock() // im so sorry
        }

        // at this point, holding the lock only does bad.

        (result.handler)(hypervisor_call.func(), (arg1, arg2, arg3));
    }
}

impl HxPosedAsyncService {
    ///
    /// # Add Notify Handler
    ///
    /// Adds a new notify handler.
    ///
    /// ## Arguments
    /// handler - Handler that will be registered.
    ///
    /// ## Warning
    /// This function internally makes a service request.
    /// The plugin must be authorized before use, or an [ErrorCode::NotAllowed] will be returned.
    ///
    /// ## Returns
    /// Result from the hypervisor. See [HypervisorError]
    pub fn add_notify_handler(
        &mut self,
        handler: AsyncNotifyHandler,
    ) -> Result<(), HypervisorError> {
        let cookie = handler.cookie;
        self.handlers.push(handler);

        let req = AddAsyncHandlerRequest {
            addr: Self::async_event as *const u64 as u64,
            cookie,
        };

        match req.send() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///
    /// # Remove Notify Handler
    ///
    /// Removes a new notify handler.
    ///
    /// ## Arguments
    /// handler - Handler that will be unregistered.
    ///
    /// ## Warning
    /// This function internally makes a service request.
    /// The plugin must be authorized before use, or an [ErrorCode::NotAllowed] will be returned.
    ///
    /// ## Returns
    /// Result from the hypervisor. See [HypervisorError]
    pub fn remove_notify_handler(&mut self, async_cookie: u16) -> Result<(), HypervisorError> {
        let req = RemoveAsyncHandlerRequest {
            addr: Self::async_event as *const u64 as u64,
            cookie: async_cookie,
        };
        self.handlers.retain(|x| x.cookie != async_cookie);

        match req.send() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub const fn new() -> Self {
        let ret = Self {
            handlers: Vec::new(),
        };

        ret
    }
}
