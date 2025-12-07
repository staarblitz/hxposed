use alloc::boxed::Box;
use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::VmcallRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use wdk::println;
use wdk_sys::{HANDLE, STATUS_SUCCESS};
use wdk_sys::ntddk::{ProbeForWrite, ZwSetEvent};

pub mod process;
pub mod memory;
pub mod thread;

pub trait AsyncCommand: Any {
    fn get_service_function(&self) -> ServiceFunction;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}

fn write_and_set(result: &HypervisorResponse, result_values: *mut u64, handle: HANDLE) {
    match microseh::try_seh(|| unsafe { ProbeForWrite(result_values as _, 16, 1) }) {
        Ok(_) => unsafe {
            result_values.write(result.result.into_bits() as u64);
            result_values.offset(1).write(result.arg1);
            result_values.offset(2).write(result.arg2);
            result_values.offset(3).write(result.arg3);
        }
        Err(x) => {
            println!("Failed to write to user buffer: {}", x);
        }
    }

    match unsafe { ZwSetEvent(handle as _, Default::default()) } {
        STATUS_SUCCESS => {}
        err => {
            println!("Failed to set event: {}", err);
        }
    }
}
