use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::security::OpenTokenRequest;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use uuid::Uuid;
use crate::plugins::commands::{write_and_set, AsyncCommand};

pub struct OpenTokenAsyncCommand {
    pub command: OpenTokenRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for OpenTokenAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::OpenToken
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