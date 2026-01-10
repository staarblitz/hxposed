use hxposed_core::services::types::security_fields::{ImpersonationLevel, TokenPrivilege, TokenType};
use crate::nt::{get_access_token_field, get_logon_session_field, AccessTokenField, LogonSessionField, PSEP_LOGON_SESSION_REFERENCES, _SEP_TOKEN_PRIVILEGES};
use crate::utils::handlebox::HandleBox;
use wdk_sys::ntddk::{ObOpenObjectByPointer, ObfDereferenceObject};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{
    SeTokenObjectType, HANDLE, NTSTATUS, PACCESS_TOKEN, PUNICODE_STRING,
    STATUS_SUCCESS, TOKEN_ALL_ACCESS, UNICODE_STRING,
};

pub struct NtToken {
    pub nt_token: PACCESS_TOKEN,
    pub account_name: PUNICODE_STRING,
    pub uid: u64,
    pub owns: bool,
}

impl Drop for NtToken {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                ObfDereferenceObject(self.nt_token as _);
            }
        }
    }
}

impl NtToken {
    pub fn from_ptr(token: PACCESS_TOKEN) -> Self {
        Self::open_thread(token, false)
    }

    fn open_thread(ptr: PACCESS_TOKEN, owns: bool) -> Self {
        Self {
            nt_token: ptr,
            uid: ptr as _,
            account_name: unsafe {
                get_logon_session_field::<UNICODE_STRING>(
                    LogonSessionField::AccountName,
                    *get_access_token_field::<PSEP_LOGON_SESSION_REFERENCES>(
                        AccessTokenField::LogonSession,
                        ptr,
                    ),
                )
            },
            owns,
        }
    }

    pub fn open_handle(&self) -> Result<HandleBox, NTSTATUS> {
        let mut handle = HANDLE::default();
        match unsafe {
            ObOpenObjectByPointer(
                self.nt_token as _,
                0,
                Default::default(),
                TOKEN_ALL_ACCESS,
                *SeTokenObjectType,
                KernelMode as _,
                &mut handle,
            )
        } {
            STATUS_SUCCESS => Ok(HandleBox::new(handle)),
            err => Err(err),
        }
    }

    pub fn get_source_name(&self) -> u64 {
        unsafe { *get_access_token_field::<u64>(AccessTokenField::TokenSource, self.nt_token) }
    }

    pub fn get_type(&self) -> TokenType {
        unsafe{*get_access_token_field::<TokenType>(AccessTokenField::Type, self.nt_token) }
    }

    pub fn get_mandatory_policy(&self) -> u32 {
        unsafe{*get_access_token_field::<u32>(AccessTokenField::MandatoryPolicy, self.nt_token)}
    }

    pub fn get_integrity_level_index(&self) -> u32 {
        unsafe {*get_access_token_field::<u32>(AccessTokenField::IntegrityLevelIndex, self.nt_token)}
    }

    pub fn get_impersonation_level(&self) -> ImpersonationLevel {
        unsafe {*get_access_token_field::<ImpersonationLevel>(AccessTokenField::ImpersonationLevel, self.nt_token)}
    }

    fn get_privileges(&self) -> &mut _SEP_TOKEN_PRIVILEGES {
        unsafe{
            &mut *get_access_token_field::<_SEP_TOKEN_PRIVILEGES>(AccessTokenField::Privileges, self.nt_token)
        }
    }

    pub fn get_enabled_privileges(&self) -> TokenPrivilege {
        self.get_privileges().Enabled.clone()
    }

    pub fn get_default_enabled_privileges(&self) -> TokenPrivilege {
        self.get_privileges().EnabledByDefault.clone()
    }

    pub fn get_present_privileges(&self) -> TokenPrivilege {
        self.get_privileges().Present.clone()
    }

    pub fn set_enabled_privileges(&mut self, new_privs: TokenPrivilege) {
        self.get_privileges().Enabled = new_privs;
    }
}
