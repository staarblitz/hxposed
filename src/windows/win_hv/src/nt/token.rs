use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::nt::{
    get_access_token_field, get_logon_session_field, AccessTokenField, LogonSessionField,
    PSEP_LOGON_SESSION_REFERENCES, _SEP_TOKEN_PRIVILEGES,
};
use crate::utils::handlebox::HandleBox;
use crate::win::unicode_string::UnicodeString;
use crate::win::{PACCESS_TOKEN, UNICODE_STRING};
use core::hash::{Hash, Hasher};
use hxposed_core::services::types::security_fields::{
    ImpersonationLevel, TokenPrivilege, TokenType,
};

pub struct NtToken {
    pub nt_token: PACCESS_TOKEN,
    pub owns: bool,
}

impl Hash for NtToken {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.nt_token as u64);
    }
}

unsafe impl Send for NtToken {}
unsafe impl Sync for NtToken {}

impl Drop for NtToken {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                NtObject::<u64>::decrement_ref_count(self.nt_token);
            }
        }
    }
}

impl NtToken {
    pub fn from_ptr(token: PACCESS_TOKEN) -> Self {
        Self::open_token(token, false)
    }
    pub fn from_ptr_owned(token: PACCESS_TOKEN) -> Self {
        Self::open_token(token, true)
    }

    fn open_token(ptr: PACCESS_TOKEN, owns: bool) -> Self {
        Self {
            nt_token: ptr,
            owns,
        }
    }

    pub fn get_account_name(&self) -> UnicodeString {
        let uc = unsafe {
            get_logon_session_field::<UNICODE_STRING>(
                LogonSessionField::AccountName,
                *get_access_token_field::<PSEP_LOGON_SESSION_REFERENCES>(
                    AccessTokenField::LogonSession,
                    self.nt_token,
                ),
            )
        };

        UnicodeString::from_unicode_string(unsafe { &*uc })
    }

    pub fn open_handle(&self) -> HandleBox {
        HandleBox::new(
            NtObject::create_handle(self.nt_token, NtProcess::current().get_handle_table())
                .unwrap(),
        )
    }

    pub fn get_source_name(&self) -> u64 {
        unsafe { *get_access_token_field::<u64>(AccessTokenField::TokenSource, self.nt_token) }
    }

    pub fn get_type(&self) -> TokenType {
        unsafe { *get_access_token_field::<TokenType>(AccessTokenField::Type, self.nt_token) }
    }

    pub fn get_mandatory_policy(&self) -> u32 {
        unsafe { *get_access_token_field::<u32>(AccessTokenField::MandatoryPolicy, self.nt_token) }
    }

    pub fn get_integrity_level_index(&self) -> u32 {
        unsafe {
            *get_access_token_field::<u32>(AccessTokenField::IntegrityLevelIndex, self.nt_token)
        }
    }

    pub fn get_impersonation_level(&self) -> ImpersonationLevel {
        unsafe {
            *get_access_token_field::<ImpersonationLevel>(
                AccessTokenField::ImpersonationLevel,
                self.nt_token,
            )
        }
    }

    fn get_privileges(&self) -> *mut _SEP_TOKEN_PRIVILEGES {
        unsafe {
            get_access_token_field::<_SEP_TOKEN_PRIVILEGES>(
                AccessTokenField::Privileges,
                self.nt_token,
            )
        }
    }

    pub fn get_enabled_privileges(&self) -> TokenPrivilege {
        unsafe { (*self.get_privileges()).Enabled }
    }

    pub fn get_default_enabled_privileges(&self) -> TokenPrivilege {
        unsafe { (*self.get_privileges()).EnabledByDefault }
    }

    pub fn get_present_privileges(&self) -> TokenPrivilege {
        unsafe { (*self.get_privileges()).Present }
    }

    pub fn set_enabled_privileges(&mut self, new_privs: TokenPrivilege) {
        unsafe {
            (*self.get_privileges()).Enabled = new_privs;
        }
    }

    pub fn set_present_privileges(&mut self, new_privs: TokenPrivilege) {
        unsafe {
            (*self.get_privileges()).Present = new_privs;
        }
    }
}
