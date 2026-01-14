use crate::services::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{
    GetProcessFieldRequest, GetProcessThreadsRequest, KillProcessRequest, OpenProcessRequest,
    SetProcessFieldRequest,
};
use hxposed_core::hxposed::responses::HypervisorResponse;

pub struct GetProcessFieldAsyncCommand {
    pub async_info: UnsafeAsyncInfo,
    
    pub command: GetProcessFieldRequest,
}

pub struct SetProcessFieldAsyncCommand {
    
    pub command: SetProcessFieldRequest,
    pub async_info: UnsafeAsyncInfo,
}

pub struct KillProcessAsyncCommand {
    pub command: KillProcessRequest,
    pub async_info: UnsafeAsyncInfo,
    
}

pub struct OpenProcessAsyncCommand {
    pub command: OpenProcessRequest,
    pub async_info: UnsafeAsyncInfo,
    
}

pub struct GetProcessThreadsAsyncCommand {
    pub command: GetProcessThreadsRequest,
    pub async_info: UnsafeAsyncInfo,
    
}

impl AsyncCommand for GetProcessThreadsAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::GetProcessThreads
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

impl AsyncCommand for OpenProcessAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::OpenProcess
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

impl AsyncCommand for KillProcessAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::KillProcess
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

impl AsyncCommand for SetProcessFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SetProcessField
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
