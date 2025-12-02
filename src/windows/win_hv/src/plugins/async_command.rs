use crate::nt::context::ApcProcessContext;
use crate::plugins::plugin::Plugin;
use core::any::Any;
use core::sync::atomic::{AtomicPtr, Ordering};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{KillProcessRequest, ProcessField};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::{AsyncInfo, UnsafeAsyncInfo};
use wdk::println;
use wdk_sys::ntddk::{ProbeForWrite, ZwSetEvent};
use wdk_sys::{HANDLE, PEPROCESS, STATUS_SUCCESS};

pub trait AsyncCommand: Any {
    fn get_service_function(&self) -> ServiceFunction;
    fn complete(&mut self, result: HypervisorResponse);
    fn as_any(&self) -> &dyn Any;
}

/// TODO: organize in submodules
pub struct GetProcessFieldAsyncCommand {
    pub process: PEPROCESS,
    pub field: ProcessField,
    pub async_info: UnsafeAsyncInfo,
    pub plugin_process: PEPROCESS,

    pub user_buffer_len: u16,
    pub user_buffer: AtomicPtr<u8>,
}

impl GetProcessFieldAsyncCommand {
    pub fn new(
        plugin_process: PEPROCESS,
        process: PEPROCESS,
        field: ProcessField,
        user_buffer_len: u16,
        user_buffer: AtomicPtr<u8>,
        async_info: UnsafeAsyncInfo,
    ) -> GetProcessFieldAsyncCommand {
        Self {
            process,
            field,
            plugin_process,
            async_info,
            user_buffer,
            user_buffer_len,
        }
    }
}

pub struct KillProcessAsyncCommand {
    pub exit_code: u32,
    pub process: PEPROCESS,
    pub async_info: UnsafeAsyncInfo,
    pub plugin_process: PEPROCESS,
}

impl KillProcessAsyncCommand {
    pub fn new(
        plugin_process: PEPROCESS,
        exit_code: u32,
        process: PEPROCESS,
        async_info: UnsafeAsyncInfo,
    ) -> Self {
        Self {
            exit_code,
            process,
            plugin_process,
            async_info
        }
    }
}

impl AsyncCommand for KillProcessAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillProcess
    }
    fn complete(&mut self, result: HypervisorResponse) {
        write_and_set(
            &result,
            self.async_info.result_values as *mut _,
            self.async_info.handle as _,
        );
        // seems like our user mode app tried to be a little too smart.
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl AsyncCommand for GetProcessFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::GetProcessField
    }

    fn complete(&mut self, result: HypervisorResponse) {
        write_and_set(
            &result,
            self.async_info.result_values as *mut _,
            self.async_info.handle as _,
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
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
