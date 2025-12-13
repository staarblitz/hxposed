use core::any::Any;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use uuid::Uuid;
use crate::plugins::commands::{write_and_set, AsyncCommand};

pub struct OpenThreadAsyncCommand {
    pub command: OpenThreadRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

pub struct SuspendResumeThreadAsyncCommand {
    pub command: SuspendResumeThreadRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

pub struct GetThreadFieldAsyncCommand {
    pub command: GetThreadFieldRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

pub struct SetThreadFieldAsyncCommand {
    pub command: SetThreadFieldRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

pub struct KillThreadAsyncCommand {
    pub command: KillThreadRequest,
    pub uuid: Uuid,
    pub async_info: UnsafeAsyncInfo,
}

impl AsyncCommand for SetThreadFieldAsyncCommand {
    fn get_service_function(&self) -> ServiceFunction {
        ServiceFunction::SetThreadField
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
    fn get_service_function(&self) -> ServiceFunction { ServiceFunction::SuspendResumeThread }

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