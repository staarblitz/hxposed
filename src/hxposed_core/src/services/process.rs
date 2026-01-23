#![allow(unused_parens)]
#![allow(dead_code)]

use crate::error::HypervisorError;
use crate::hxposed::requests::process::*;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::process::GetProcessFieldResponse;
use crate::hxposed::ObjectType;
use crate::intern::win::GetCurrentProcessId;
use crate::services::memory::HxMemory;
use crate::services::security::HxToken;
use crate::services::types::process_fields::*;
use alloc::string::String;
use alloc::vec::Vec;
use core::arch::asm;

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

impl HxProcess {
    pub fn system() -> Self {
        Self::open(4).unwrap()
    }

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
    pub fn open_handle(id: u32) -> Result<u64, HypervisorError> {
        let result = OpenProcessRequest {
            process_id: id,
            open_type: ObjectOpenType::Handle,
        }
        .send()?;

        Ok(match result.object {
            ObjectType::Handle(handle) => handle,
            _ => panic!("Unexpected object type"),
        })
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
    pub fn get_primary_token(&self) -> Result<HxToken, HypervisorError> {
        match (GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Token(0),
        }
        .send()?)
        .field
        {
            ProcessField::Token(addr) => Ok(HxToken::from_raw_object(addr)?),
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
    pub fn swap_token(&self, token: &HxToken) -> Result<EmptyResponse, HypervisorError> {
        SetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Token(token.addr),
        }
        .send()
    }

    ///
    /// # Set Mitigation Options
    ///
    /// Sets the internal `MitigationFlags1` and `MitigationFlags2` fields of `_EPROCESS`.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::PROCESS_EXECUTIVE`]
    ///
    pub fn set_mitigation_options(
        &self,
        options: MitigationOptions,
    ) -> Result<EmptyResponse, HypervisorError> {
        SetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::MitigationFlags(options),
        }
        .send()
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
            field: ProcessField::MitigationFlags(MitigationOptions::default()),
        }
        .send()?;

        match result.field {
            ProcessField::MitigationFlags(x) => Ok(MitigationOptions::from(x)),
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
    pub fn get_threads(&self) -> Result<&[u32], HypervisorError> {
        match (GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Threads(0),
        })
        .send()?.field
        {
            ProcessField::Threads(offset) => unsafe {
                let length = *((0x20090000 + offset) as *const usize);
                Ok(core::slice::from_raw_parts(
                    (0x20090000usize + (offset as usize) + size_of::<usize>()) as *const u32,
                    length,
                ))
            },
            _ => unreachable!(),
        }
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
            memory: HxMemory {
                process: call.object.into_raw().1,
            },
            addr: call.object.into_raw().1,
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
    ) -> Result<EmptyResponse, HypervisorError> {
        SetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Protection(new_protection),
        }
        .send()
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
    ) -> Result<EmptyResponse, HypervisorError> {
        SetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::Signers(new_levels),
        }
        .send()
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
            field: ProcessField::Signers(ProcessSignatureLevels::default()),
        }
        .send()?;

        match result.field {
            ProcessField::Signers(signers) => Ok(signers),
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
            field: ProcessField::Protection(ProcessProtection::default()),
        }
        .send()?;

        match result.field {
            ProcessField::Protection(signers) => Ok(signers),
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
    pub fn get_nt_path(&self) -> Result<String, HypervisorError> {
        match (GetProcessFieldRequest {
            process: self.addr,
            field: ProcessField::NtPath(0),
        })
        .send()?.field
        {
            ProcessField::NtPath(offset) => unsafe {
                //TODO: Move this into a helper function
                let length = *((0x20090000 + offset) as *const u32);
                Ok(String::from_utf16(core::slice::from_raw_parts(
                    (0x20090000usize + (offset as usize) + size_of::<u32>()) as *const u16,
                    length as _,
                ))
                .unwrap())
            },
            _ => unreachable!(),
        }
    }

    /*///
    /// # Kill
    ///
    /// Uses `PspTerminateProces` internally to terminate the process object.
    ///
    /// Consumes the object.
    ///
    /// ## Warning
    /// * Object must be dropped by hand **after** a successful termination.
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
    ///             drop(process);
    ///         }
    ///         Err(e) => {
    ///             println!("Error killing process: {:?}", e);
    ///         }
    ///     }
    /// ```
    pub fn kill(&self, exit_code: u32) -> AsyncFuture<KillProcessRequest, EmptyResponse> {
        KillProcessRequest {
            process: self.addr,
            exit_code,
        }
        .send_async()
    }*/
}
