use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::nt::token::NtToken;
use crate::nt::SYSTEM_TOKEN;
use core::sync::atomic::Ordering;
use hxposed_core::hxposed::call::ServiceParameter;
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::process::ObjectOpenType;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::responses::empty::{EmptyResponse, OpenObjectResponse};
use hxposed_core::hxposed::responses::security::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::ObjectType;
use wdk_sys::PACCESS_TOKEN;

pub(crate) fn set_token_field_sync(request: SetTokenFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();

    let mut token = match process
        .get_object_tracker_unchecked()
        .get_open_token(request.token)
    {
        Some(x) => x,
        None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
    };

    match request.field {
        TokenField::EnabledPrivileges(privs) => {
            token.set_enabled_privileges(privs);
            EmptyResponse::with_service(ServiceFunction::SetTokenField)
        }
        _ => HypervisorResponse::invalid_params(ServiceParameter::Function),
    }
}
pub(crate) fn get_token_field_sync(request: GetTokenFieldRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let state = process.get_hx_async_state_unchecked();
    let token = match process
        .get_object_tracker_unchecked()
        .get_open_token(request.token as _)
    {
        Some(x) => x,
        None if request.token == 0 => match request.field {
            // asking for SYSTEM token
            TokenField::PresentPrivileges(_) => {
                &mut NtToken::from_ptr(SYSTEM_TOKEN.load(Ordering::Relaxed) as PACCESS_TOKEN)
            }
            _ => return HypervisorResponse::not_found_what(NotFoundReason::Token),
        },
        None => return HypervisorResponse::not_found_what(NotFoundReason::Token),
    };

    match request.field {
        TokenField::SourceName(_) => GetTokenFieldResponse::SourceName(token.get_source_name()),
        TokenField::AccountName(_) => {
            let field = token.get_account_name();
            let raw_string = field.get_raw_bytes();
            let offset = state.write_result(&raw_string.len(), 1);
            state.write_result(raw_string.as_ptr(), raw_string.len());
            GetTokenFieldResponse::AccountName(offset)
        }
        TokenField::Type(_) => GetTokenFieldResponse::Type(token.get_type()),
        TokenField::IntegrityLevelIndex(_) => {
            GetTokenFieldResponse::IntegrityLevelIndex(token.get_integrity_level_index())
        }
        TokenField::MandatoryPolicy(_) => {
            GetTokenFieldResponse::MandatoryPolicy(token.get_mandatory_policy())
        }
        TokenField::ImpersonationLevel(_) => {
            GetTokenFieldResponse::ImpersonationLevel(token.get_impersonation_level())
        }
        TokenField::EnabledPrivileges(_) => {
            GetTokenFieldResponse::EnabledPrivileges(token.get_enabled_privileges())
        }
        TokenField::PresentPrivileges(_) => {
            GetTokenFieldResponse::PresentPrivileges(token.get_present_privileges())
        }
        TokenField::EnabledByDefaultPrivileges(_) => {
            GetTokenFieldResponse::EnabledByDefaultPrivileges(
                token.get_default_enabled_privileges(),
            )
        }
    }
    .into_raw()
}

pub(crate) fn open_token_sync(request: OpenTokenRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    match request.open_type {
        ObjectOpenType::Handle => {
            match NtObject::<PACCESS_TOKEN>::create_handle(
                request.token as _,
                process.get_handle_table(),
            ) {
                Ok(handle) => OpenObjectResponse {
                    object: ObjectType::Handle(handle as _),
                }
                .into_raw(),
                Err(_) => HypervisorResponse::not_found_what(NotFoundReason::Token),
            }
        }
        ObjectOpenType::Hypervisor => {
            // we need more checking honestly
            process
                .get_object_tracker_unchecked()
                .add_open_token(NtToken::from_ptr_owned(request.token as _));
            EmptyResponse::with_service(ServiceFunction::OpenToken)
        }
    }
}

pub(crate) fn close_token_sync(request: CloseTokenRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    process
        .get_object_tracker_unchecked()
        .pop_open_token(request.token as _);

    EmptyResponse::with_service(ServiceFunction::CloseToken)
}
