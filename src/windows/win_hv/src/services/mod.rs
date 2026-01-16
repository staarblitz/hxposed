use crate::services::callback_services::{
    await_notification, register_callback, unregister_callback,
};
use crate::services::memory_services::*;
use crate::services::process_services::*;
use crate::services::security_services::*;
use crate::services::thread_services::*;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::requests::notify::{
    AwaitNotificationRequest, RegisterNotifyHandlerRequest, UnregisterNotifyHandlerRequest,
};
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::HypervisorResponse;

mod callback_services;
pub(crate) mod commands;
pub mod memory_services;
pub mod process_services;
pub mod security_services;
pub mod thread_services;

pub fn handle_callback_services(
    request: &HypervisorRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::RegisterNotifyEvent => {
            register_callback(RegisterNotifyHandlerRequest::from_raw(request), async_info)
        }
        ServiceFunction::UnregisterNotifyEvent => unregister_callback(
            UnregisterNotifyHandlerRequest::from_raw(request),
            async_info,
        ),
        ServiceFunction::AwaitNotifyEvent => {
            await_notification(AwaitNotificationRequest::from_raw(request), async_info)
        }
        _ => unreachable!(),
    }
}

pub fn handle_thread_services(
    request: &HypervisorRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenThread => {
            open_thread_async(OpenThreadRequest::from_raw(request), async_info)
        }
        ServiceFunction::SuspendResumeThread => {
            suspend_resume_thread_async(SuspendResumeThreadRequest::from_raw(request), async_info)
        }
        ServiceFunction::KillThread => {
            kill_thread_async(KillThreadRequest::from_raw(request), async_info)
        }
        ServiceFunction::GetThreadField => {
            get_thread_field_async(GetThreadFieldRequest::from_raw(request), async_info)
        }
        ServiceFunction::SetThreadField => {
            set_thread_field_async(SetThreadFieldRequest::from_raw(request), async_info)
        }
        ServiceFunction::CloseThread => close_thread(CloseThreadRequest::from_raw(request)),
        _ => unreachable!(),
    }
}

pub fn handle_security_services(
    request: &HypervisorRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenToken => {
            open_token_async(OpenTokenRequest::from_raw(request), async_info)
        }
        ServiceFunction::CloseToken => {
            close_token_sync(CloseTokenRequest::from_raw(request), async_info)
        }
        ServiceFunction::SetTokenField => {
            set_token_field_async(SetTokenFieldRequest::from_raw(request), async_info)
        }
        ServiceFunction::GetTokenField => {
            get_token_field_async(GetTokenFieldRequest::from_raw(request), async_info)
        }
        _ => unreachable!("forgot to implement this one"),
    }
}

///
/// # Handle Memory Services
///
/// Dispatches the memory service request to [memory_services].
///
pub fn handle_memory_services(
    request: &HypervisorRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !request.call.is_async() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    match request.call.func() {
        ServiceFunction::ProcessVMOperation => {
            process_vm_operation_async(RWProcessMemoryRequest::from_raw(request), async_info)
        }
        ServiceFunction::ProtectProcessMemory => {
            protect_vm_async(ProtectProcessMemoryRequest::from_raw(request), async_info)
        }
        ServiceFunction::AllocateMemory => {
            allocate_mdl_async(AllocateMemoryRequest::from_raw(request), async_info)
        }
        ServiceFunction::MapMemory => {
            map_mdl_async(MapMemoryRequest::from_raw(request), async_info)
        }
        ServiceFunction::FreeMemory => {
            free_mdl_async(FreeMemoryRequest::from_raw(request), async_info)
        }
        _ => unreachable!("forgot to implement this one"),
    }
}

///
/// # Handle Process Services
///
/// Dispatches the process service request to [process_services].
///
pub fn handle_process_services(
    request: &HypervisorRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenProcess => {
            open_process_async(OpenProcessRequest::from_raw(request), async_info)
        }
        ServiceFunction::CloseProcess => close_process(CloseProcessRequest::from_raw(request)),
        ServiceFunction::GetProcessField => {
            get_process_field_async(GetProcessFieldRequest::from_raw(request), async_info)
        }
        ServiceFunction::SetProcessField => {
            set_process_field_async(SetProcessFieldRequest::from_raw(request), async_info)
        }
        ServiceFunction::GetProcessThreads => {
            get_process_threads_async(GetProcessThreadsRequest::from_raw(request), async_info)
        }
        ServiceFunction::KillProcess => {
            kill_process_async(KillProcessRequest::from_raw(request), async_info)
        }
        _ => unreachable!(),
    }
}
