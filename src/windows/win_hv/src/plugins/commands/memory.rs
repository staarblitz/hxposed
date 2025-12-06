use crate::plugins::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{
    ProtectProcessMemoryRequest, RWProcessMemoryRequest,
};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::PEPROCESS;

pub struct RWProcessMemoryAsyncCommand {
    pub plugin_process: PEPROCESS,
    pub process: PEPROCESS,
    pub command: RWProcessMemoryRequest,
    pub async_info: UnsafeAsyncInfo,
}

pub struct ProtectProcessMemoryAsyncCommand {
    pub plugin_process: PEPROCESS,
    pub process: PEPROCESS,
    pub command: ProtectProcessMemoryRequest,
    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for ProtectProcessMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::ProtectProcessMemory
    }

    fn complete(&mut self, result: HypervisorResponse) {
        write_and_set(
            &result,
            self.async_info.result_values as *mut _,
            self.async_info.handle as _,
        )
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
