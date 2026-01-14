use crate::nt::probe;
use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::responses::HypervisorResponse;
use wdk_sys::ntddk::ZwSetEvent;
use wdk_sys::{HANDLE, STATUS_SUCCESS};
use hxposed_core::events::UnsafeAsyncInfo;

pub mod callback;
pub mod memory;
pub mod process;
pub mod security;
pub mod thread;

pub trait AsyncCommand: Any + Send + Sync {
    fn get_service_function(&self) -> ServiceFunction;
    fn get_async_info(&self) -> &UnsafeAsyncInfo;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}

fn write_and_set(result: &HypervisorResponse, result_values: *mut u64, handle: HANDLE) {
    match probe::probe_for_write(result_values as _, 16, 1) {
        Ok(_) => unsafe {
            result_values.write(result.result.into_bits() as u64);
            result_values.offset(1).write(result.arg1);
            result_values.offset(2).write(result.arg2);
            result_values.offset(3).write(result.arg3);
        },
        Err(_) => {
            log::error!("Failed to write to user buffer");
        }
    }

    match unsafe { ZwSetEvent(handle as _, Default::default()) } {
        STATUS_SUCCESS => {}
        err => {
            log::error!("Failed to set event: {}", err);
        }
    }
}
