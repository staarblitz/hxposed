use alloc::vec;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::async_help::{AddAsyncHandlerRequest, RemoveAsyncHandlerRequest};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::AsyncNotifyHandler;
use crate::plugins::plugin::Plugin;

pub fn add_async_handler(
    guest: &mut dyn Guest,
    request: AddAsyncHandlerRequest,
    plugin: &'static mut Plugin,
) -> HypervisorResponse {
    if plugin.handlers.len() > 47 {
        return HypervisorResponse::not_allowed(NotAllowedReason::Unknown);
    }

    match plugin.add_notify_handler(AsyncNotifyHandler {
        filter: vec![],
        cookie: request.cookie,
        handler: request.addr as *mut  _ as _
    }) {
        Ok(_) => {
            EmptyResponse::with_service(ServiceFunction::AddAsyncHandler)
        }
        Err(_) => {
            HypervisorResponse::not_allowed(NotAllowedReason::Unknown)
        }
    }
}

pub fn remove_async_handler(
    guest: &mut dyn Guest,
    request: RemoveAsyncHandlerRequest,
    plugin: &'static mut Plugin
) -> HypervisorResponse {
    plugin.remove_notify_handler(request.cookie);

    EmptyResponse::with_service(ServiceFunction::RemoveAsyncHandler)
}