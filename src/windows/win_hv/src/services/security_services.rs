use crate::plugins::commands::security::*;
use crate::plugins::{Plugin, PluginTable};
use alloc::boxed::Box;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::security::*;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use hxposed_core::services::types::security_fields::{ImpersonationLevel, TokenType};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{ObReferenceObjectByPointer, ProbeForWrite};
use wdk_sys::{STATUS_SUCCESS, SeTokenObjectType, TOKEN_ALL_ACCESS, PVOID, UNICODE_STRING};
use crate::nt::{get_access_token_field, get_logon_session_field, AccessTokenField, LogonSessionField, PSEP_LOGON_SESSION_REFERENCES, _SEP_LOGON_SESSION_REFERENCES};

pub(crate) fn get_token_field_sync(request: &GetTokenFieldAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

    let token = match plugin
        .object_table
        .get_open_token(request.command.addr as _)
    {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
    };

    match request.command.field {
        TokenField::SourceName => {
            let field = unsafe {
                *get_access_token_field::<u64>(AccessTokenField::TokenSource, token)
            };

            GetTokenFieldResponse::SourceName(field)
        }
        TokenField::AccountName => {
            let logon_session = unsafe {
                *get_access_token_field::<PSEP_LOGON_SESSION_REFERENCES>(AccessTokenField::LogonSession, token)
            };

            let account_name = unsafe {
                &mut *get_logon_session_field::<UNICODE_STRING>(LogonSessionField::AccountName, logon_session)
            };

            if request.command.data_len == 0 {
                GetTokenFieldResponse::AccountName(account_name.Length)
            } else {
                match microseh::try_seh(|| unsafe {
                    ProbeForWrite(request.command.data as _, request.command.data_len as _, 1)
                }) {
                    Ok(_) => {
                        unsafe {
                            account_name.Buffer.copy_to_nonoverlapping(request.command.data as _, account_name.Length as usize / 2)
                        }

                        GetTokenFieldResponse::AccountName(account_name.Length)
                    },
                    Err(_) => return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser)
                }
            }

        }
        TokenField::Type => {
            let field = unsafe {
                *get_access_token_field::<TokenType>(AccessTokenField::Type, token)
            };

            GetTokenFieldResponse::Type(field)
        }
        TokenField::IntegrityLevelIndex => {
            let field = unsafe {
                *get_access_token_field::<u32>(AccessTokenField::Type, token)
            };

            GetTokenFieldResponse::IntegrityLevelIndex(field)
        }
        TokenField::MandatoryPolicy => {
            let field = unsafe {
                *get_access_token_field::<u32>(AccessTokenField::Type, token)
            };

            GetTokenFieldResponse::MandatoryPolicy(field)
        }
        TokenField::ImpersonationLevel => {
            let field = unsafe {
                *get_access_token_field::<ImpersonationLevel>(AccessTokenField::Type, token)
            };

            GetTokenFieldResponse::ImpersonationLevel(field)
        }
        _ => GetTokenFieldResponse::Unknown,
    }.into_raw()
}

pub(crate) fn get_token_field_async(
    guest: &mut dyn Guest,
    request: GetTokenFieldRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !plugin.perm_check(PluginPermissions::SECURITY_MANAGE) {
        return HypervisorResponse::not_allowed_perms(PluginPermissions::SECURITY_MANAGE);
    }

    let obj = GetTokenFieldAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            plugin.queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::GetTokenField)
        }
        false => match obj.command.field {
            TokenField::AccountName => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
            _ => get_token_field_sync(&obj),
        },
    }
}

pub(crate) fn open_token_sync(request: &OpenTokenAsyncCommand) -> HypervisorResponse {
    let plugin = match PluginTable::lookup(request.uuid) {
        Some(plugin) => plugin,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Plugin),
    };

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

pub(crate) fn open_token_async(
    guest: &mut dyn Guest,
    request: OpenTokenRequest,
    plugin: &mut Plugin,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    plugin.queue_command(Box::new(OpenTokenAsyncCommand {
        command: request,
        uuid: plugin.uuid,
        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::OpenToken)
}
