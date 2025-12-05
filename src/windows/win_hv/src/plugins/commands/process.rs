use core::any::Any;
use core::sync::atomic::AtomicPtr;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{GetProcessFieldRequest, KillProcessRequest, ProcessField, RWProcessMemoryRequest, SetProcessFieldRequest};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk::println;
use wdk_sys::ntddk::{ProbeForWrite, ZwSetEvent};
use wdk_sys::{HANDLE, PEPROCESS, STATUS_SUCCESS};
use crate::plugins::commands::AsyncCommand;

pub struct GetProcessFieldAsyncCommand {
    pub process: PEPROCESS,
    pub async_info: UnsafeAsyncInfo,
    pub plugin_process: PEPROCESS,
    pub command: GetProcessFieldRequest,
}

pub struct SetProcessFieldAsyncCommand {
    pub process: PEPROCESS,
    pub command: SetProcessFieldRequest,
    pub async_info: UnsafeAsyncInfo,
    pub plugin_process: PEPROCESS,
}

pub struct KillProcessAsyncCommand {
    pub command: KillProcessRequest,
    pub process: PEPROCESS,
    pub async_info: UnsafeAsyncInfo,
    pub plugin_process: PEPROCESS,
}

pub struct RWProcessMemoryAsyncCommand {
    pub plugin_process: PEPROCESS,
    pub process: PEPROCESS,
    pub command: RWProcessMemoryRequest,
    pub async_info: UnsafeAsyncInfo,
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

impl AsyncCommand for SetProcessFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SetProcessField
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

impl AsyncCommand for RWProcessMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::ProcessVMOperation
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
