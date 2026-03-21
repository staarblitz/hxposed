#![allow(dead_code)]
#![allow(unused_parens)]

use crate::error::HypervisorError;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::thread::*;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::thread::GetThreadFieldResponse;
use crate::intern::win::GetCurrentThreadId;
use crate::services::security::HxToken;

pub struct HxThread {
    pub id: u32,
    addr: u64,
}

impl Drop for HxThread {
    fn drop(&mut self) {
        let _ = CloseThreadRequest {
            thread: self.addr,
        }
        .send();
    }
}

impl HxThread {
    ///
    /// # Current
    ///
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
    ///
    /// Opens the current thread for your use.
    pub fn current() -> Result<Self, HypervisorError> {
        Self::open(unsafe { GetCurrentThreadId() })
    }

    ///
    /// # Swap Impersonation Token
    ///
    /// Swaps the impersonation token of the thread.
    ///
    /// ## Warning
    /// - This happens while the thread is EXECUTING.
    /// - The results can be disastrous.
    /// - This isn't supported in any way.
    /// - You have been warned.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_SECURITY`]
    /// - This function does not require [`PluginPermissions::SECURITY_MANAGE`], but you will need it for obtaining a HxToken.
    ///
    /// ## Arguments
    /// - `token` - New token. See [`HxToken`]
    pub async fn swap_impersonation_token(
        &self,
        token: &HxToken,
    ) -> Result<EmptyResponse, HypervisorError> {
        SetThreadFieldRequest {
            thread: self.addr,
            field: ThreadField::AdjustedClientToken(token.addr),
        }.send()
    }

    ///
    /// # Get Impersonation Token
    ///
    /// Gets the `AdjustedClientToken` field from `_ETHREAD` structure.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
    /// * [`PluginPermissions::THREAD_SECURITY`]
    ///
    /// ## Return
    /// * [`HxToken`] - Impersonation token.
    /// * [`HypervisorError`] - Most likely thread is not impersonating.
    pub async fn get_impersonation_token(&self) -> Result<HxToken, HypervisorError> {
        match (GetThreadFieldRequest {
            thread: self.addr,
            field: ThreadField::AdjustedClientToken(0),
        }).send()?
        {
            GetThreadFieldResponse::AdjustedClientToken(x) => {
                Ok(HxToken::from_raw_object(x)?)
            }
            _ => unreachable!(),
        }
    }

    ///
    /// # Is Impersonating
    ///
    /// Checks the `ActiveImpersonationInfo` from `CrossThreadFlags` in `_ETHREAD` structure.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
    /// * [`PluginPermissions::THREAD_SECURITY`]
    ///
    /// ## Return
    /// * [`bool`]
    pub fn is_impersonating(&self) -> Result<bool, HypervisorError> {
        match (GetThreadFieldRequest {
            thread: self.addr,
            field: ThreadField::ActiveImpersonationInfo(false),
        }
        .send()?)
        {
            GetThreadFieldResponse::ActiveImpersonationInfo(x) => Ok(x),
            _ => unreachable!(),
        }
    }

    ///
    /// # Open
    ///
    /// Opens a thread.
    ///
    /// ## Arguments
    /// * `id` - Thread id
    ///
    /// ## Permissions
    /// - [`PluginPermissions::THREAD_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`Result`] containing [`HxThread`] or error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let process = HxThread::open(4).unwrap();
    /// ```
    pub fn open(id: u32) -> Result<Self, HypervisorError> {
        let result = OpenThreadRequest {
            tid: id as _,
        }
        .send()?;

        Ok(Self {
            id,
            addr: result.object.into(),
        })
    }
}
