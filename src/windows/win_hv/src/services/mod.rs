use crate::services::process_services::*;
use crate::services::security_services::*;
use crate::services::thread_services::*;
use crate::services::memory_services::*;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::*;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::requests::thread::*;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::requests::io::MsrIoRequest;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::requests::notify::{RegisterNotifyHandlerRequest, UnregisterNotifyHandlerRequest};
use hxposed_core::hxposed::responses::HypervisorResponse;
use crate::services::callback_services::{register_callback_receiver, unregister_callback_receiver};
use crate::services::io_services::rw_msr;

mod callback_services;
pub mod memory_services;
pub mod process_services;
pub mod security_services;
pub mod thread_services;
mod io_services;
/*pub fn cancel_async_call(request: &HypervisorRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let request = CancelAsyncCallRequest::from_raw(request);

    process.get_hx_async_state().cancel(request.Cookie);

    EmptyResponse::with_service(ServiceFunction::CancelAsyncCall)
}*/

pub fn handle_cpu_io_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::MsrIo => rw_msr(MsrIoRequest::from_raw(request)),
        _ => unreachable!()
    }
}

pub fn handle_callback_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::UnregisterNotifyEvent => unregister_callback_receiver(UnregisterNotifyHandlerRequest::from_raw(request)),
        ServiceFunction::RegisterNotifyEvent => register_callback_receiver(RegisterNotifyHandlerRequest::from_raw(request)),
        _ => unreachable!()
    }
}

pub fn handle_thread_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenThread => open_thread_sync(OpenThreadRequest::from_raw(request)),
        ServiceFunction::SuspendResumeThread => HypervisorResponse::not_found(),
        ServiceFunction::KillThread => HypervisorResponse::not_found(),
        ServiceFunction::GetThreadField => {
            get_thread_field_sync(GetThreadFieldRequest::from_raw(request))
        }
        ServiceFunction::SetThreadField => {
            set_thread_field_sync(SetThreadFieldRequest::from_raw(request))
        }
        ServiceFunction::CloseThread => close_thread_sync(CloseThreadRequest::from_raw(request)),
        _ => unreachable!(),
    }
}

pub fn handle_security_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenToken => open_token_sync(OpenTokenRequest::from_raw(request)),
        ServiceFunction::CloseToken => close_token_sync(CloseTokenRequest::from_raw(request)),
        ServiceFunction::SetTokenField => {
            set_token_field_sync(SetTokenFieldRequest::from_raw(request))
        }
        ServiceFunction::GetTokenField => {
            get_token_field_sync(GetTokenFieldRequest::from_raw(request))
        }
        _ => unreachable!("forgot to implement this one"),
    }
}

///
/// # Handle Memory Services
///
/// Dispatches the memory service request to [memory_services].
///
pub fn handle_memory_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::GetSetPageAttribute => get_set_page_attribute(PageAttributeRequest::from_raw(request)),
        ServiceFunction::MapVaToPa => map_va_to_pa(MapVaToPaRequest::from_raw(request)),
        ServiceFunction::AllocateMemory => allocate_memory(AllocateMemoryRequest::from_raw(request)),
        ServiceFunction::FreeMemory => free_memory(FreeMemoryRequest::from_raw(request)),
        _ => unreachable!(),
    }
}

///
/// # Handle Process Services
///
/// Dispatches the process service request to [process_services].
///
pub fn handle_process_services(request: &HypervisorRequest) -> HypervisorResponse {
    match request.call.func() {
        ServiceFunction::OpenProcess => open_process(OpenProcessRequest::from_raw(request)),
        ServiceFunction::CloseProcess => close_process(CloseProcessRequest::from_raw(request)),
        ServiceFunction::GetProcessField => {
            get_process_field_sync(GetProcessFieldRequest::from_raw(request))
        }
        ServiceFunction::SetProcessField => {
            set_process_field_sync(SetProcessFieldRequest::from_raw(request))
        }
        ServiceFunction::GetProcessThreads => HypervisorResponse::not_found(),
        ServiceFunction::KillProcess => kill_process_sync(KillProcessRequest::from_raw(request)),
        _ => unreachable!(),
    }
}
