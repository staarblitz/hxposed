use crate::plugins::commands::{write_and_set, AsyncCommand};
use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::{
    GetProcessFieldRequest, KillProcessRequest,
    SetProcessFieldRequest,
};
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::PEPROCESS;

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
