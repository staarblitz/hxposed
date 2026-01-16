use crate::services::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::HypervisorResponse;

pub struct OpenThreadAsyncCommand {
    pub command: OpenThreadRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct SuspendResumeThreadAsyncCommand {
    pub command: SuspendResumeThreadRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct GetThreadFieldAsyncCommand {
    pub command: GetThreadFieldRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct SetThreadFieldAsyncCommand {
    pub command: SetThreadFieldRequest,

    pub async_info: UnsafeAsyncInfo,
}

pub struct KillThreadAsyncCommand {
    pub command: KillThreadRequest,

    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for SetThreadFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SetThreadField
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

impl AsyncCommand for GetThreadFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::GetThreadField
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

impl AsyncCommand for KillThreadAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillThread
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

impl AsyncCommand for SuspendResumeThreadAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SuspendResumeThread
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

impl AsyncCommand for OpenThreadAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::OpenThread
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
