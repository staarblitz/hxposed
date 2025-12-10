use crate::plugins::commands::security::*;
use crate::plugins::{Plugin, PluginTable};
use alloc::boxed::Box;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::security::OpenTokenRequest;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::HypervisorResponse;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use wdk_sys::ntddk::{ObReferenceObjectByPointer, ProbeForWrite};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{SeTokenObjectType, STATUS_SUCCESS, TOKEN_ALL_ACCESS};

pub(crate) fn open_token_sync(request: &OpenTokenAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    // extra paranoid
    match microseh::try_seh(|| unsafe { ProbeForWrite(request.command.addr as _, 8, 1) }) {
        Ok(_) => {
            match unsafe {
                // verify object exists
                ObReferenceObjectByPointer(
                    request.command.addr as _,
                    TOKEN_ALL_ACCESS,
                    *SeTokenObjectType,
                    KernelMode as _,
                )
            } {
                STATUS_SUCCESS => {
                    plugin
                        .object_table
                        .open_tokens
                        .push(request.command.addr as _);
                    EmptyResponse::with_service(ServiceFunction::OpenToken)
                }
                err => HypervisorResponse::nt_error(err as _),
            }
        }
        Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
    }
}

pub(crate) fn open_token_async(
    guest: &mut dyn Guest,
    request: OpenTokenRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    plugin.queue_command(Box::new(OpenTokenAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::OpenToken)
}
