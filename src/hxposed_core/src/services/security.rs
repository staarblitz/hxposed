use alloc::string::String;
use alloc::vec::Vec;
use crate::error::HypervisorError;
use crate::hxposed::requests::security::*;
use crate::hxposed::requests::Vmcall;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::types::security_fields::*;
use crate::hxposed::requests::security::*;
use crate::hxposed::responses::security::GetTokenFieldResponse;

pub struct HxToken {
   addr: u64,
}

impl HxToken {
    pub(crate) async fn from_raw_object(addr: u64) -> Result<HxToken, HypervisorError> {
        let result = OpenTokenRequest {
            addr
        }.send_async().await?;

        Ok(Self {
            addr: result.addr
        })
    }

    ///
    /// # Get Source Name
    ///
    /// Gets the `SourceName` field in `_TOKEN_SOURCE` structure.
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetTokenFieldResponse::SourceName`]. Which it should NOT.
    ///
    /// ## Permission
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Return
    /// * [`String`] - A beautiful string.
    pub async fn get_source_name(&self) -> Result<String, HypervisorError> {
        match(GetTokenFieldRequest {
            addr: self.addr,
            field: TokenField::SourceName,
            ..Default::default()
        }).send_async().await? {
            GetTokenFieldResponse::SourceName(name) => { // did I tell this u64 is a char[8]?
                match String::from_utf8(name.to_le_bytes().into_vec()) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(HypervisorError::not_found())
                }
            }
            _ => unreachable!()
        }
    }

    ///
    /// # Get Account Name
    ///
    /// Gets the account name associated with the token. (See [[Warning]])
    ///
    /// ## Warning
    /// - For some reason, this does **NOT** return the account associated with the token. It returns the name of the local computer.
    /// - E.g., not "SYSTEM" or "Admin", "DESKTOP-ASD1541HAF" instead.
    /// - Function purely named as such because the `_TOKEN`'s `LogonSession` field has this field named as `AccountName`.
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
            addr: self.addr,
            field: TokenField::AccountName,
            ..Default::default()
        }).send_async().await? {
            GetTokenFieldResponse::AccountName(len) => bytes = len,
            _ => unreachable!()
        };

        let mut buffer = Vec::<u16>::with_capacity(bytes as usize / 2);
        assert_eq!(buffer.capacity(), bytes as usize / 2);

        match (GetTokenFieldRequest {
            addr: self.addr,
            field: TokenField::AccountName,
            data: buffer.as_mut_ptr() as _,
            data_len: buffer.capacity() as _,
        }).send_async().await? {
            GetTokenFieldResponse::AccountName(length) => {
                assert_eq!(length, bytes);

                unsafe {
                    buffer.set_len(bytes as usize / 2);
                }

                match String::from_utf16(buffer.as_slice()) {
                    Ok(str) => Ok(str),
                    Err(_) => Err(HypervisorError::not_found()),
                }
            },
            _ => unreachable!()
        }
    }
}