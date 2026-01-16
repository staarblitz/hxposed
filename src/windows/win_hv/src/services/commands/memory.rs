use crate::services::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::HypervisorResponse;

pub struct RWProcessMemoryAsyncCommand {
    pub command: RWProcessMemoryRequest,
    pub async_info: UnsafeAsyncInfo,
}

pub struct ProtectProcessMemoryAsyncCommand {
    pub command: ProtectProcessMemoryRequest,
    pub async_info: UnsafeAsyncInfo,
}

pub struct AllocateMemoryAsyncCommand {
    pub command: AllocateMemoryRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct MapMemoryAsyncCommand {
    pub command: MapMemoryRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct FreeMemoryAsyncCommand {
    pub command: FreeMemoryRequest,

    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for FreeMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::FreeMemory
    }

    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
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

impl AsyncCommand for MapMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::MapMemory
    }

    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
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

impl AsyncCommand for AllocateMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::AllocateMemory
    }
    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
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

impl AsyncCommand for ProtectProcessMemoryAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::ProtectProcessMemory
    }
    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
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
    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
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
