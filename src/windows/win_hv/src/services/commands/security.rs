use crate::services::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::responses::HypervisorResponse;

pub struct OpenTokenAsyncCommand {
    pub command: OpenTokenRequest,
    
    pub async_info: UnsafeAsyncInfo,
}

pub struct GetTokenFieldAsyncCommand {
    pub command: GetTokenFieldRequest,
    
    pub async_info: UnsafeAsyncInfo,
}

pub struct SetTokenFieldAsyncCommand {
    pub command: SetTokenFieldRequest,
    
    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for SetTokenFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SetTokenField
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

impl AsyncCommand for GetTokenFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::GetTokenField
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

impl AsyncCommand for OpenTokenAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::OpenToken
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
