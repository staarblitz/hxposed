use crate::nt::token::NtToken;
use crate::nt::{SYSTEM_TOKEN, probe};
use crate::objects::ObjectTracker;
use crate::services::commands::security::*;
use crate::utils::pop_guard::PopGuard;
use alloc::boxed::Box;
use core::sync::atomic::Ordering;
use hv::hypervisor::host::Guest;
use hxposed_core::events::UnsafeAsyncInfo;
use hxposed_core::hxposed::ObjectType;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::security::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::services::types::security_fields::TokenPrivilege;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{ObOpenObjectByPointer, ObReferenceObjectByPointer};
use wdk_sys::{HANDLE, PACCESS_TOKEN, STATUS_SUCCESS, SeTokenObjectType, TOKEN_ALL_ACCESS};

pub(crate) fn set_token_field_sync(request: &SetTokenFieldAsyncCommand) -> HypervisorResponse {
    let mut token = match ObjectTracker::get_open_token(request.command.token as _) {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
    };

    match request.command.field {
        TokenField::EnabledPrivileges => {
            if request.command.data_len != size_of::<TokenPrivilege>() {
                return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
            }

            match probe::probe_for_read(request.command.data as _, request.command.data_len as _) {
                Ok(_) => {
                    let user_field = unsafe { &*(request.command.data as *mut TokenPrivilege) };
                    token.set_enabled_privileges(user_field.clone());

                    EmptyResponse::with_service(ServiceFunction::SetTokenField)
                }
                Err(_) => HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
            }
        }
        _ => HypervisorResponse::invalid_params(ServiceParameter::Function),
    }
}

pub(crate) fn set_token_field_async(
    request: SetTokenFieldRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    let obj = SetTokenFieldAsyncCommand {
        command: request,

        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            ObjectTracker::queue_command(Box::new(obj));

            EmptyResponse::with_service(ServiceFunction::SetTokenField)
        }
        false => match obj.command.field {
            _ => HypervisorResponse::invalid_params(ServiceParameter::IsAsync),
        },
    }
}

pub(crate) fn get_token_field_sync(request: &GetTokenFieldAsyncCommand) -> HypervisorResponse {
    let token = match ObjectTracker::get_open_token(request.command.token as _) {
        Some(x) => x,
        None if request.command.token == 0 => match request.command.field {
            // asking for SYSTEM token
            TokenField::PresentPrivileges => PopGuard::no_src(NtToken::from_ptr(
                SYSTEM_TOKEN.load(Ordering::Relaxed) as PACCESS_TOKEN,
            )),
            _ => return HypervisorResponse::not_found_what(NotFoundReason::Token),
        },
        None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
    };

    match request.command.field {
        TokenField::SourceName => GetTokenFieldResponse::SourceName(token.get_source_name()),
        TokenField::AccountName => {
            let account_name = unsafe { &*token.account_name };
            if request.command.data_len == 0 {
                GetTokenFieldResponse::AccountName(account_name.Length)
            } else {
                match probe::probe_for_write(
                    request.command.data as _,
                    request.command.data_len as _,
                ) {
                    Ok(_) => {
                        unsafe {
                            account_name.Buffer.copy_to_nonoverlapping(
                                request.command.data as _,
                                account_name.Length as usize / 2,
                            )
                        }

                        GetTokenFieldResponse::AccountName(account_name.Length)
                    }
                    Err(_) => {
                        return HypervisorResponse::invalid_params(ServiceParameter::BufferByUser);
                    }
                }
            }
        }
        TokenField::Type => GetTokenFieldResponse::Type(token.get_type()),
        TokenField::IntegrityLevelIndex => {
            GetTokenFieldResponse::IntegrityLevelIndex(token.get_integrity_level_index())
        }
        TokenField::MandatoryPolicy => {
            GetTokenFieldResponse::MandatoryPolicy(token.get_mandatory_policy())
        }
        TokenField::ImpersonationLevel => {
            GetTokenFieldResponse::ImpersonationLevel(token.get_impersonation_level())
        }
        TokenField::EnabledPrivileges => {
            GetTokenFieldResponse::EnabledPrivileges(token.get_enabled_privileges())
        }
        TokenField::PresentPrivileges => {
            GetTokenFieldResponse::PresentPrivileges(token.get_present_privileges())
        }
        TokenField::EnabledByDefaultPrivileges => {
            GetTokenFieldResponse::EnabledByDefaultPrivileges(
                token.get_default_enabled_privileges(),
            )
        }
        _ => return HypervisorResponse::not_found(),
    }
    .into_raw()
}

pub(crate) fn get_token_field_async(
    request: GetTokenFieldRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    let obj = GetTokenFieldAsyncCommand {
        command: request,

        async_info,
    };

    match obj.async_info.is_present() {
        true => {
            ObjectTracker::queue_command(Box::new(obj));
            EmptyResponse::with_service(ServiceFunction::GetTokenField)
        }
        false => match obj.command.field {
            TokenField::AccountName => {
                HypervisorResponse::invalid_params(ServiceParameter::IsAsync)
            }
            _ => get_token_field_sync(&obj),
        },
    }
}

pub(crate) fn open_token_sync(request: &OpenTokenAsyncCommand) -> HypervisorResponse {
    match request.command.open_type {
        ObjectOpenType::Handle => {
            let mut handle = HANDLE::default();
            match unsafe {
                ObOpenObjectByPointer(
                    request.command.token as _,
                    0,
                    Default::default(),
                    TOKEN_ALL_ACCESS,
                    *SeTokenObjectType,
                    KernelMode as _,
                    &mut handle,
                )
            } {
                STATUS_SUCCESS => OpenObjectResponse {
                    object: ObjectType::Handle(handle as _),
                }
                .into_raw(),
                err => HypervisorResponse::nt_error(err as _),
            }
        }
        ObjectOpenType::Hypervisor => {
            match unsafe {
                // verify object exists
                ObReferenceObjectByPointer(
                    request.command.token as _,
                    TOKEN_ALL_ACCESS,
                    *SeTokenObjectType,
                    KernelMode as _,
                )
            } {
                STATUS_SUCCESS => {
                    ObjectTracker::add_open_token(NtToken::from_ptr(request.command.token as _));

                    // ObDereferenceObject.....

                    EmptyResponse::with_service(ServiceFunction::OpenToken)
                }
                err => HypervisorResponse::nt_error(err as _),
            }
        }
    }
}

pub(crate) fn open_token_async(
    request: OpenTokenRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if !async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    ObjectTracker::queue_command(Box::new(OpenTokenAsyncCommand {
        command: request,

        async_info,
    }));

    EmptyResponse::with_service(ServiceFunction::OpenToken)
}

pub(crate) fn close_token_sync(
    request: CloseTokenRequest,
    async_info: UnsafeAsyncInfo,
) -> HypervisorResponse {
    if async_info.is_present() {
        return HypervisorResponse::invalid_params(ServiceParameter::IsAsync);
    }

    ObjectTracker::get_open_token(request.token as _).take();

    EmptyResponse::with_service(ServiceFunction::CloseToken)
}
