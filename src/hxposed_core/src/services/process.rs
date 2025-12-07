use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::{CloseProcessRequest, GetProcessFieldRequest, GetProcessThreadsRequest, KillProcessRequest, OpenProcessRequest, ProcessField, ObjectOpenType, SetProcessFieldRequest};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::process::{GetProcessFieldResponse, GetProcessThreadsResponse};
use crate::intern::win::GetCurrentProcessId;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::async_service::AsyncPromise;
use crate::services::memory::HxMemory;
use crate::services::types::process_fields::{ProcessProtection, ProcessSignatureLevels};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::pin::Pin;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct HxProcess {
    pub id: u32,
    pub memory: HxMemory,
    addr: AtomicU64,
}

impl Drop for HxProcess {
    fn drop(&mut self) {
        let _ = CloseProcessRequest {
            addr: self.addr.load(Ordering::Relaxed),
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
    /// ## Returns
    /// * Handle as an u64.
    pub async fn open_handle(id: u32) -> Result<u64, HypervisorError> {
        let result = OpenProcessRequest {
            process_id: id,
            open_type: ObjectOpenType::Handle,
        }.send_async().await?;

        Ok(result.addr)
    }

    ///
    /// # Get Threads
    ///
    /// Iterates over the threads of the process object.
    ///
    /// ## Warning
    /// This temporarily locks the process object for safe access. (You probably don't care, just saying in case you do.)
    ///
    /// ## Returns
    /// * [`Vec<u32>`] - Vector containing the ids of threads under specified process.
    pub async fn get_threads(&self) -> Result<Vec<u32>, HypervisorError> {
        let result = GetProcessThreadsRequest {
            id: self.id,
            data: 0 as _,
            data_len: 0,
        }.send_async().await?;

        let mut buffer = Vec::<u32>::with_capacity(result.number_of_threads as _);

        let result = GetProcessThreadsRequest {
            id: self.id,
            data: buffer.as_mut_ptr() as _,
            data_len: (buffer.capacity() as i32 * 4) as _,
        }.send_async().await?;

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
            memory: HxMemory { id },
            addr: AtomicU64::new(call.addr),
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
        mut new_protection: ProcessProtection,
    ) -> Future<SetProcessFieldRequest, EmptyResponse> {
        SetProcessFieldRequest::set_protection(self.id, &mut new_protection).send_async()
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
        mut new_levels: ProcessSignatureLevels,
    ) -> Future<SetProcessFieldRequest, EmptyResponse> {
        SetProcessFieldRequest::set_signature_levels(self.id, &mut new_levels).send_async()
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
            id: self.id,
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
            id: self.id,
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
            id: self.id,
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
            id: self.id,
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
    /// Uses *PspTerminateProcess* internally to terminate the process object.
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
    /// - [Result] with most likely an NT error.
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
            id: self.id,
            exit_code,
        }
        .send_async()
    }
}
