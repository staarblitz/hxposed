use core::any::Any;
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::KillProcessRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::AsyncInfo;
use wdk_sys::ntddk::{KeSetEvent, ObReferenceObjectByHandle, ObfDereferenceObject};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{ExEventObjectType, EVENT_ALL_ACCESS, FALSE, KEVENT, PEPROCESS};

pub trait AsyncCommand: Any {
    fn get_service_function(&self) -> ServiceFunction;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}

pub struct KillProcessAsyncCommand {
    pub call: KillProcessRequest,
    pub exit_code: u32,
    pub process: PEPROCESS,
    pub event: KEVENT,
    pub info: AsyncInfo,
}

impl KillProcessAsyncCommand {
    pub fn new(call: KillProcessRequest, process: PEPROCESS, async_info: &AsyncInfo) -> Self {
        let mut event = KEVENT::default();
        let _ = unsafe {
            ObReferenceObjectByHandle(
                async_info.handle as _,
                EVENT_ALL_ACCESS,
                *ExEventObjectType,
                KernelMode as _,
                &mut event as *mut _ as _,
                Default::default(),
            )
        };

        Self {
            exit_code: call.exit_code,
            process,
            call,
            event,
            info: async_info.clone(),
        }
    }
}

impl Drop for KillProcessAsyncCommand {
    fn drop(&mut self) {
        unsafe { ObfDereferenceObject(&mut self.event as *mut _ as _) };
    }
}

impl AsyncCommand for KillProcessAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillProcess
    }
    fn complete(&mut self, result: HypervisorResponse) {
        if let Ok(_) = microseh::try_seh(|| {
            let ptr = &mut unsafe { *self.info.result_values.load(Ordering::Relaxed) };
            ptr[0] = result.result.into_bits() as u64;
            ptr[1] = result.arg1;
            ptr[2] = result.arg2;
            ptr[3] = result.arg3;
        }) {
            unsafe { KeSetEvent(&mut self.event, 0, FALSE as _) };
        }

        // seems like our user mode app tried to be a little too smart.
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
