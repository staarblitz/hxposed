use crate::services::commands::{AsyncCommand, write_and_set};
use core::any::Any;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::notify::AwaitNotificationRequest;
use hxposed_core::hxposed::responses::notify::AwaitNotificationResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

pub struct AwaitNotificationRequestAsyncCommand {
    pub async_info: UnsafeAsyncInfo,
    pub command: AwaitNotificationRequest,
    pub response: HypervisorResponse,
}

impl AsyncCommand for AwaitNotificationRequestAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::AwaitNotifyEvent
    }

    fn get_async_info(&self) -> &UnsafeAsyncInfo {
        &self.async_info
    }

    fn complete(&mut self, _result: HypervisorResponse) {
        write_and_set(
            &self.response,
            self.async_info.result_values as *mut _,
            self.async_info.handle as _,
        );
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
