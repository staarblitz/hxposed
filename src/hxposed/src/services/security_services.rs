use crate::nt;
use crate::nt::process::NtProcess;
use crate::nt::token::NtToken;
use crate::utils::logger::{HxLogger, LogEvent, LogType};
use hxposed_core::hxposed::error::NotFoundReason;
use hxposed_core::hxposed::requests::security::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::security::*;
use hxposed_core::hxposed::responses::{HxResponse, OpenObjectResponse, SyscallResponse};
use hxposed_core::hxposed::{ObjectType, TokenObject};

pub(crate) fn set_token_field_sync(request: SetTokenFieldRequest) -> HxResponse {
    let process = NtProcess::current();
    let token = match process
        .get_object_tracker_unchecked()
        .get_open_token(request.token)
    {
        Some(x) => x,
        None => return HxResponse::not_found_what(NotFoundReason::Token),
    };

    match request.field {
        TokenField::EnabledPrivileges(privs) => {
            token.set_enabled_privileges(privs);
            EmptyResponse::default()
        }
        TokenField::PresentPrivileges(privs) => {
            token.set_present_privileges(privs);
            EmptyResponse::default()
        }
        _ => HxResponse::invalid_params(0),
    }
}

#[allow(static_mut_refs)]
pub(crate) fn get_token_field_sync(request: GetTokenFieldRequest) -> HxResponse {
    let process = NtProcess::current();
    let token = match process
        .get_object_tracker_unchecked()
        .get_open_token(request.token as _)
    {
        Some(x) => x,
        None => return HxResponse::not_found_what(NotFoundReason::Token),
    };

    match request.field {
        TokenField::Unknown => return HxResponse::invalid_params(0),
        TokenField::SourceName(_) => GetTokenFieldResponse::SourceName(token.get_source_name()),
        TokenField::AccountName(ptr) => {
            let field = token.get_account_name();
            if ptr != 0 {
                let _ = microseh::try_seh(|| unsafe {
                    core::ptr::copy_nonoverlapping(field.as_ptr(), ptr as _, field.len())
                });
            }
            GetTokenFieldResponse::AccountName(field.len() as _)
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

pub(crate) fn open_token_sync(request: OpenTokenRequest) -> HxResponse {
    let process = NtProcess::current();
    let token = match request.token == 0 {
        true => unsafe { nt::SYSTEM_TOKEN as TokenObject },
        false => request.token,
    };
    HxLogger::serial_log(
        LogType::Trace,
        LogEvent::TrackObject(token, process.nt_process as _),
    );

    OpenObjectResponse {
        object: ObjectType::Token(
            process
                .get_object_tracker_unchecked()
                .add_open_token(NtToken::from_ptr_owning(request.token as _)),
        ),
    }
    .into_raw()
}

pub(crate) fn close_token_sync(request: CloseTokenRequest) -> HxResponse {
    let process = NtProcess::current();
    match process
        .get_object_tracker_unchecked()
        .pop_open_token(request.token as _)
    {
        None => return HxResponse::not_found_what(NotFoundReason::Token),
        Some(_) => {}
    }
    HxLogger::serial_log(
        LogType::Trace,
        LogEvent::DetrackObject(request.token, process.nt_process as _),
    );
    EmptyResponse::default()
}
