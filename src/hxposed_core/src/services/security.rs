use crate::error::HypervisorError;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::security::*;
use crate::hxposed::requests::security::*;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::security::GetTokenFieldResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::types::security_fields::*;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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
    pub(crate) async fn from_raw_object(addr: u64) -> Result<HxToken, HypervisorError> {
        OpenTokenRequest {
            token: addr,
            open_type: ObjectOpenType::Hypervisor,
        }
        .send_async()
        .await?;

        Ok(Self { addr })
    }

    ///
    /// # Open Handle
    ///
    /// Returns a handle with `TOKEN_ALL_ACCESS`.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Warning
    /// - The caller holds full ownership to the handle.
    ///
    /// ## Returns
    /// * Handle as an u64.
    pub(crate) async fn open_handle(&self) -> Result<u64, HypervisorError> {
        let resp = OpenTokenRequest {
            token: self.addr,
            open_type: ObjectOpenType::Handle,
        }
        .send_async()
        .await?;

        Ok(resp.object.into())
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
    pub fn get_present_privileges(&self) -> Result<TokenPrivilege, HypervisorError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::PresentPrivileges,
            ..Default::default()
        }
        .send()?)
        {
            GetTokenFieldResponse::PresentPrivileges(privileges) => Ok(privileges),
            _ => unreachable!(),
        }
    }

    ///
    /// # Get System Present Privileges
    ///
    /// - Gets the valid privilege bitmask for the SYSTEM token.
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
    pub fn get_system_present_privileges() -> Result<TokenPrivilege, HypervisorError> {
        match (GetTokenFieldRequest {
            token: 0,
            field: TokenField::PresentPrivileges,
            ..Default::default()
        }
            .send()?)
        {
            GetTokenFieldResponse::PresentPrivileges(privileges) => Ok(privileges),
            _ => unreachable!(),
        }
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
    pub async fn set_enabled_privileges(
        &self,
        privileges: TokenPrivilege,
    ) -> Result<EmptyResponse, HypervisorError> {
        let mut boxed = Box::new(privileges);
        SetTokenFieldRequest::set_enabled_privileges(self.addr, boxed.as_mut())
            .send_async()
            .await
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
    pub fn get_enabled_privileges(&self) -> Result<TokenPrivilege, HypervisorError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::EnabledPrivileges,
            ..Default::default()
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
    pub fn get_default_enabled_privileges(&self) -> Result<TokenPrivilege, HypervisorError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::EnabledByDefaultPrivileges,
            ..Default::default()
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
    pub async fn get_source_name(&self) -> Result<String, HypervisorError> {
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::SourceName,
            ..Default::default()
        })
        .send_async()
        .await?
        {
            GetTokenFieldResponse::SourceName(name) => {
                // did I tell this u64 is a char[8]?
                match String::from_utf8(name.to_le_bytes().to_vec()) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(HypervisorError::not_found()),
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
    pub async fn get_account_name(&self) -> Result<String, HypervisorError> {
        let mut bytes = 0u16;
        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::AccountName,
            ..Default::default()
        })
        .send_async()
        .await?
        {
            GetTokenFieldResponse::AccountName(len) => bytes = len,
            _ => unreachable!(),
        };

        let mut buffer = Vec::<u16>::with_capacity(bytes as usize / 2);
        assert_eq!(buffer.capacity(), bytes as usize / 2);

        match (GetTokenFieldRequest {
            token: self.addr,
            field: TokenField::AccountName,
            data: buffer.as_mut_ptr() as _,
            data_len: buffer.capacity() as _,
        })
        .send_async()
        .await?
        {
            GetTokenFieldResponse::AccountName(length) => {
                assert_eq!(length, bytes);

                unsafe {
                    buffer.set_len(bytes as usize / 2);
                }

                match String::from_utf16(buffer.as_slice()) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(HypervisorError::not_found()),
                }
            }
            _ => unreachable!(),
        }
    }
}
