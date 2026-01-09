use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::*;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::process::GetProcessFieldResponse;
use crate::intern::win::GetCurrentProcessId;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::async_service::AsyncPromise;
use crate::services::memory::HxMemory;
use crate::services::security::HxToken;
use crate::services::types::process_fields::*;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::pin::Pin;
use core::ptr::null_mut;

#[derive(Debug)]
pub struct HxProcess {
    pub id: u32,
    pub memory: HxMemory,
    pub(crate) addr: u64,
}

impl Drop for HxProcess {
    fn drop(&mut self) {
        let _ = CloseProcessRequest {
            process: self.addr,
            open_type: ObjectOpenType::Hypervisor,
        }
        .send();
    }
}

pub type Future<T, X> = Pin<Box<AsyncPromise<T, X>>>;

impl HxProcess {
    ///
    /// # Current
    ///
    /// Opens the current process for your use.
    pub fn current() -> Result<Self, HypervisorError> {
        Self::open(unsafe { GetCurrentProcessId() })
    }

    ///
    /// # Open Handle
    ///
    /// Returns a handle with `PROCESS_ALL_ACCESS`.
    ///
    /// Remember that you still might have to remove process protection ([`Self::set_protection`]) to have full access to the process object.
    ///
    /// ## Arguments
    /// * `id` - Process id
    ///
    /// ## Warning
    /// - The caller holds full ownership to the handle.
    ///
    /// ## Returns
    /// * Handle as an u64.
    pub async fn open_handle(id: u32) -> Result<u64, HypervisorError> {
        let result = OpenProcessRequest {
            process_id: id,
            open_type: ObjectOpenType::Handle,
        }
        .send_async()
        .await?;

        Ok(result.addr)
    }

    ///
    /// # Get Primary Token
    ///
    /// Gets `Token` field of `_EPROCESS` structure.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_SECURITY`]
    /// * [`PluginPermissions::SECURITY_MANAGE`]
    ///
    /// ## Returns
    /// * [`HxToken`]
    pub async fn get_primary_token(&self) -> Result<HxToken, HypervisorError> {
        match (GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Token,
            ..Default::default()
        }
        .send_async()
        .await?)
        {
            GetProcessFieldResponse::Token(addr) => Ok(HxToken::from_raw_object(addr).await?),
            _ => unreachable!(),
        }
    }

    ///
    /// # Swap Token
    ///
    /// Swaps the primary token of the process.
    ///
    /// ## Warning
    /// - This happens while the process is RUNNING.
    /// - The results can be disastrous.
    /// - This isn't supported in any way.
    /// - You have been warned.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_SECURITY`]
    /// - This function does not require [`PluginPermissions::SECURITY_MANAGE`], but you will need it for obtaining a HxToken.
    ///
    /// ## Arguments
    /// - `token` - New token. See [`HxToken`]
    pub async fn swap_token(&self, token: &HxToken) -> Result<EmptyResponse, HypervisorError> {
        SetProcessFieldRequest{
            process: self.addr,
            field: ProcessField::Token,
            data: token.addr as _,
            data_len: 8
        }.send_async().await
    }

    ///
    /// # Set Mitigation Options
    ///
    /// Sets the internal `MitigationFlags1` and `MitigationFlags2` fields of `_EPROCESS`.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    pub async fn set_mitigation_options(
        &self,
        options: MitigationOptions,
    ) -> Result<EmptyResponse, HypervisorError> {
        let mut boxed_options = Box::new(options);
        SetProcessFieldRequest::set_mitigation_options(self.addr, boxed_options.as_mut())
            .send_async()
            .await
    }

    ///
    /// # Get Mitigation Options
    ///
    /// Gets the internal `MitigationFlags1` and `MitigationFlags2` fields of `_EPROCESS`.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Return
    /// * [`MitigationOptions`] - Contains the mitigation flags.
    pub fn get_mitigation_options(&self) -> Result<MitigationOptions, HypervisorError> {
        let result = GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::MitigationFlags,
            ..Default::default()
        }
        .send()?;

        match result {
            GetProcessFieldResponse::Mitigation(x) => Ok(MitigationOptions::from(x)),
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Threads
    ///
    /// Iterates over the threads of the process object.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Warning
    /// This temporarily locks the process object for safe access. (You probably don't care, just saying in case you do.)
    ///
    /// ## Returns
    /// * [`Vec<u32>`] - Vector containing the ids of threads under specified process.
    pub async fn get_threads(&self) -> Result<Vec<u32>, HypervisorError> {
        let result = GetProcessThreadsRequest {
            process: self.addr,
            data: 0 as _,
            data_len: 0,
        }
        .send_async()
        .await?;

        let mut buffer = Vec::<u32>::with_capacity(result.number_of_threads as _);

        let result = GetProcessThreadsRequest {
            process: self.addr,
            data: buffer.as_mut_ptr() as _,
            data_len: (buffer.capacity() as i32 * 4) as _,
        }
        .send_async()
        .await?;

        assert_eq!(buffer.capacity(), result.number_of_threads as _);

        unsafe {
            buffer.set_len(result.number_of_threads as _);
        }

        Ok(buffer)
    }

    ///
    /// # Open
    ///
    /// Opens a process.
    ///
    /// ## Arguments
    /// * `id` - Process id
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`Result`] containing [`HxProcess`] or error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let process = HxProcess::open(4).unwrap();
    /// ```
    pub fn open(id: u32) -> Result<Self, HypervisorError> {
        let call = OpenProcessRequest {
            process_id: id,
            open_type: ObjectOpenType::Hypervisor,
        }
        .send()?;

        Ok(Self {
            id,
            memory: HxMemory { process: call.addr },
            addr: call.addr,
        })
    }

    ///
    /// # Set Protection
    ///
    /// Sets the internal process protection object. The `_PS_PROTECTION`.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`EmptyResponse`] - Empty. You can use [`Self::get_protection`] to check the operation if you have anxiety problems.
    ///
    /// ## Example
    ///
    /// ```rust
    /// match process.set_protection(
    ///         ProcessProtection::new()
    ///             .with_audit(false)
    ///             .with_protection_type(ProtectionType::None)
    ///             .with_signer(ProtectionSigner::None),
    ///     ).await {
    ///         Ok(_) => println!("Process protection changed!"),
    ///         Err(x) => println!("Error changing process protection: {:?}", x),
    ///     }
    /// ```
    pub fn set_protection(
        &mut self,
        new_protection: ProcessProtection,
    ) -> Future<SetProcessFieldRequest, EmptyResponse> {
        let mut boxed_protection = Box::new(new_protection);
        SetProcessFieldRequest::set_protection(self.addr, boxed_protection.as_mut()).send_async()
    }

    ///
    /// # Set Signature Level(s)
    ///
    /// Sets the internal process protection object. The `SignatureLevel` and `SectionSignatureLevel`.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`EmptyResponse`] - Empty.
    ///
    /// ## Example
    ///
    /// ```rust
    ///  match process
    ///         .set_signature_levels(
    ///             ProcessSignatureLevels::new()
    ///                 .with_signature_level(ProcessSignatureLevel::AntiMalware)
    ///                 .with_section_signature_level(0),
    ///         )
    ///         .await
    ///     {
    ///         Ok(_) => println!("Process signature levels changed!"),
    ///         Err(x) => println!("Error changing process signature levels: {:?}", x),
    ///     }
    /// ```
    pub fn set_signature_levels(
        &mut self,
        new_levels: ProcessSignatureLevels,
    ) -> Future<SetProcessFieldRequest, EmptyResponse> {
        let mut boxed_levels = Box::new(new_levels);
        SetProcessFieldRequest::set_signature_levels(self.addr, boxed_levels.as_mut()).send_async()
    }

    ///
    /// # Get Signature Levels
    ///
    /// - Gets the internal process signature levels. The `SignatureLevel` and `SectionSignatureLevel`.
    /// - For more information about signature levels, protection and so on and so forth,
    /// please visit [this article](https://www.crowdstrike.com/en-us/blog/protected-processes-part-3-windows-pki-internals-signing-levels-scenarios-signers-root-keys/).
    ///
    /// (You can also visit [this blog](https://staarblitz.github.io). I heard there is pretty neat stuff there.)
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetProcessFieldResponse::Signers`]. Which it SHOULD NOT.
    /// - Issue a bug report if you observe a panic.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`ProcessSignatureLevels`] - Signature levels (both `SignatureLevel` and `SectionSignatureLevel`)
    /// * [`HypervisorError`] - Most likely an NT side error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let signature = process.get_signature_levels().unwrap();
    /// ```
    pub fn get_signature_levels(&self) -> Result<ProcessSignatureLevels, HypervisorError> {
        let result = GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Signers,
            ..Default::default()
        }
        .send()?;

        match result {
            GetProcessFieldResponse::Signers(signers) => {
                Ok(ProcessSignatureLevels::from_bits(signers))
            }
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Protection
    ///
    /// Gets the internal process protection object. The `_PS_PROTECTION`.
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetProcessFieldResponse::Protection`]. Which it SHOULD NOT.
    /// - Issue a bug report if you observe a panic.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// * [`ProcessProtection`] - Full path of the process.
    /// * [`HypervisorError`] - Most likely an NT side error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let protection = process.get_protection().unwrap();
    /// ```
    pub fn get_protection(&self) -> Result<ProcessProtection, HypervisorError> {
        let result = GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Protection,
            ..Default::default()
        }
        .send()?;

        match result {
            GetProcessFieldResponse::Protection(protection) => {
                Ok(ProcessProtection::from_bits(protection as _))
            }
            _ => unreachable!(),
        }
    }

    ///
    /// # Get Nt Path
    ///
    /// Gets the Nt path of the process.
    ///
    /// E.g. it starts with (\\?\), not C:.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetProcessFieldResponse::NtPath`]. Which it SHOULD NOT.
    /// - Issue a bug report if you observe a panic.
    ///
    /// ## Return
    /// * [`String`] - Full path of the process.
    /// * [`HypervisorError::not_found`] - Unable to decode string from UTF16.
    pub async fn get_nt_path(&self) -> Result<String, HypervisorError> {
        let mut bytes = 0u16;

        let mut promise = GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::NtPath,
            data: null_mut(),
            data_len: 0,
        }
        .send_async();

        match promise.await {
            Ok(resp) => match resp {
                GetProcessFieldResponse::NtPath(length) => {
                    bytes = length;
                }
                _ => unreachable!(),
            },
            Err(e) => return Err(e),
        }

        let mut buffer = Vec::<u16>::with_capacity(bytes as usize / 2);
        assert_eq!(buffer.capacity(), bytes as usize / 2);

        let mut promise = GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::NtPath,
            data: buffer.as_mut_ptr() as *mut u8,
            data_len: buffer.capacity() as _,
        }
        .send_async();

        match promise.await {
            Ok(resp) => match resp {
                GetProcessFieldResponse::NtPath(length) => {
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
            },
            Err(e) => Err(e),
        }
    }

    ///
    /// # Kill
    ///
    /// Uses [`PspTerminateProces`] internally to terminate the process object.
    ///
    /// Consumes the object.
    ///
    /// ## Arguments
    /// * `exit_code` - The [`NTSTATUS`] exit code of the process.
    ///
    /// ## Permissions
    /// - [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    /// ## Returns
    /// - [`Result`] with most likely an NT error.
    ///
    /// ## Example
    /// ```rust
    ///  match process.kill(0).await {
    ///         Ok(_) => {
    ///             println!("Killed process!");
    ///         }
    ///         Err(e) => {
    ///             println!("Error killing process: {:?}", e);
    ///         }
    ///     }
    /// ```
    pub fn kill(self, exit_code: u32) -> Future<KillProcessRequest, EmptyResponse> {
        KillProcessRequest {
            process: self.addr,
            exit_code,
        }
        .send_async()
    }
}
