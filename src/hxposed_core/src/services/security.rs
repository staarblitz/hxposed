#![allow(dead_code)]
#![allow(unused_parens)]

use crate::error::HxError;
use crate::hxposed::requests::security::*;
use crate::hxposed::requests::Syscall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::security::GetTokenFieldResponse;
use crate::hxposed::{ObjectType, TokenObject};
use crate::services::types::security_fields::TokenPrivilege;
use alloc::string::String;

#[derive(Debug)]
pub struct HxToken {
    pub(crate) addr: u64,
}

impl Drop for HxToken {
    fn drop(&mut self) {
        let _ = CloseTokenRequest { token: self.addr }.send();
    }
}

impl HxToken {
    pub(crate) fn from_raw_object(addr: TokenObject) -> Result<HxToken, HxError> {
        OpenTokenRequest { token: addr }.send()?;

        Ok(Self { addr })
    }

    ///
    /// # Open System Token
    ///
    /// Returns instance of [`Hxtoken`] with `TOKEN_ALL_ACCESS` for system token.
    ///
    /// ## Returns
    /// * [`HxToken`] for the SYSTEM
    pub fn get_system_token() -> HxToken {
        let addr = OpenTokenRequest { token: 0 }.send().unwrap();

        HxToken {
            addr: match addr.object {
                ObjectType::Token(x) => x,
                _ => unreachable!(),
            },
        }
    }

    ///
    /// # Get Present Privileges
    ///
    /// - Gets the valid privilege bitmask for this token.
    /// - Must not be confused with [`Self::get_enabled_privileges`].
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::PresentPrivileges`]. Which it should NOT.
    ///
    /// ## Return
    /// * [`TokenPrivilege`] - Bitmask of privileges.
    pub fn get_present_privileges(&self) -> Result<TokenPrivilege, HxError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::PresentPrivileges(TokenPrivilege::None),
        }
        .send()?)
        {
            GetTokenFieldResponse::PresentPrivileges(privileges) => Ok(privileges),
            _ => unreachable!(),
        }
    }

    ///
    /// # Set Present Privileges
    ///
    /// - Sets the valid privilege bitmask for this token.
    /// - Must not be confused with [`Self::set_enabled_privileges`].
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::PresentPrivileges`]. Which it should NOT.
    pub fn set_present_privileges(
        &self,
        privileges: TokenPrivilege,
    ) -> Result<EmptyResponse, HxError> {
        SetTokenFieldRequest {
            token: self.addr,
            field: TokenField::PresentPrivileges(privileges),
        }
        .send()
    }

    ///
    /// # Get Enabled Privileges
    ///
    /// Gets the privileges currently enabled for this token.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Arguments
    /// * `privileges` - New privilege bitmask to apply. See [`TokenPrivilege`].
    ///
    /// ## Warning
    /// - Make sure that new token privilege mask is compatible with present privileges (See [`Self::get_present_privileges`]).
    ///
    /// ## Return
    /// * Nothing
    pub fn set_enabled_privileges(
        &self,
        privileges: TokenPrivilege,
    ) -> Result<EmptyResponse, HxError> {
        SetTokenFieldRequest {
            token: self.addr,
            field: TokenField::EnabledPrivileges(privileges),
        }
        .send()
    }

    ///
    /// # Get Enabled Privileges
    ///
    /// Gets the privileges currently enabled for this token.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::EnabledPrivileges`]. Which it should NOT.
    ///
    /// ## Return
    /// * [`TokenPrivilege`] - Bitmask of privileges.
    pub fn get_enabled_privileges(&self) -> Result<TokenPrivilege, HxError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::EnabledPrivileges(TokenPrivilege::None),
        }
        .send()?)
        {
            GetTokenFieldResponse::EnabledPrivileges(privileges) => Ok(privileges),
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Default Enabled Privileges
    ///
    /// Gets the privileges enabled by default for this token.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::EnabledByDefaultPrivileges`]. Which it should NOT.
    ///
    /// ## Return
    /// * [`TokenPrivilege`] - Bitmask of privileges.
    pub fn get_default_enabled_privileges(&self) -> Result<TokenPrivilege, HxError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::EnabledByDefaultPrivileges(TokenPrivilege::None),
        }
        .send()?)
        {
            GetTokenFieldResponse::EnabledByDefaultPrivileges(privileges) => Ok(privileges),
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Source Name
    ///
    /// Gets the `SourceName` field in `_TOKEN_SOURCE` structure.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::SourceName`]. Which it should NOT.
    ///
    ///
    /// ## Return
    /// * [`String`] - A beautiful string.
    pub fn get_source_name(&self) -> Result<String, HxError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::SourceName(0),
        })
        .send()?
        {
            GetTokenFieldResponse::SourceName(name) => {
                // did I tell this u64 is a char[8]?
                match String::from_utf8(name.to_le_bytes().to_vec()) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(HxError::InvalidParameters(0)),
                }
            }
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Account Name
    ///
    /// Gets the account name associated with the token. (See [[Warning]])
    ///
    /// ## Warning
    /// - For some reason, this does **NOT** return the account associated with the token for SYSTEM user. It returns the name of the local computer.
    /// - E.g., not "SYSTEM", "DESKTOP-ASD1541HAF" instead.
    /// - No idea why.
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::AccountName`]. Which it should NOT.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Return
    /// * [`String`] - A beautiful string.
    pub async fn get_account_name(&self) -> Result<String, HxError> {
        let mut vec = Vec::<u8>::with_capacity(1024);
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::AccountName(vec.as_ptr() as _),
        })
        .send()?
        {
            GetTokenFieldResponse::AccountName(offset) => {
                unsafe {
                    vec.set_len(offset as _);
                }
                String::from_utf8(vec).map_err(|_| HxError::InvalidParameters(0))
            }
            _ => unreachable!(),
        }
    }
}
