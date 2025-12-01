use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::{
    CloseProcessRequest, GetProcessFieldRequest, KillProcessRequest, OpenProcessRequest,
    ProcessField, ProcessOpenType,
};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::hxposed::responses::process::GetProcessFieldResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::async_service::{
    AsyncPromise, GLOBAL_ASYNC_NOTIFY_HANDLER, HxPosedAsyncService,
};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::arch::asm;
use core::ptr::{null_mut, slice_from_raw_parts};
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};

pub struct HxProcess {
    pub id: u32,
    addr: AtomicU64,
}

impl Drop for HxProcess {
    fn drop(&mut self) {
        let _ = CloseProcessRequest {
            addr: self.addr.load(Ordering::Relaxed),
            open_type: ProcessOpenType::Hypervisor,
        }
        .send();
    }
}

impl HxProcess {
    ///
    /// # Open
    ///
    /// Opens a process.
    ///
    /// ## Arguments
    /// * `id` - Process id
    ///
    /// ## Permissions
    /// [PluginPermissions::PROCESS_EXECUTIVE]
    ///
    /// ## Returns
    /// - [Result] containing [NtProcess] or error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let process = HxProcess::open(4).unwrap();
    /// ```
    pub fn open(id: u32) -> Result<Self, HypervisorError> {
        let call = OpenProcessRequest {
            process_id: id,
            open_type: ProcessOpenType::Hypervisor,
        }
        .send()?;

        Ok(Self {
            id,
            addr: AtomicU64::new(call.addr),
        })
    }

    ///
    /// # Get Nt Path
    ///
    /// Gets the Nt path of the process.
    ///
    /// E.g. it starts with (\\?\), not C:.
    ///
    /// ## Panic
    /// - This function panics if hypervisor returns anything else than [`GetProcessFieldResponse::NtPath`]. Which it SHOULD NOT.
    /// - Issue a bug report if you observe a panic.
    ///
    /// ## Return
    /// * [`String`] - Full path of the process.
    /// * [`HypervisorError::not_found`] - Unable to decode string from UTF16.
    pub fn get_nt_path(&self) -> Result<String, HypervisorError> {
        let mut bytes = 0u16;

        let mut promise = GetProcessFieldRequest {
            id: self.id,
            field: ProcessField::NtPath,
            user_buffer: AtomicPtr::new(null_mut()),
            user_buffer_len: 0,
        }
        .send_async();

        unsafe{
            asm!("int 0x3")
        }

        match promise.wait() {
            Ok(resp) => match resp {
                GetProcessFieldResponse::NtPath(length) => {
                    bytes = length;
                }
                _ => unreachable!(),
            },
            Err(e) => return Err(e),
        }

        let mut buffer = Vec::<u16>::with_capacity(bytes as usize);

        let mut promise = GetProcessFieldRequest {
            id: self.id,
            field: ProcessField::NtPath,
            user_buffer: AtomicPtr::new(buffer.as_mut_ptr()),
            user_buffer_len: buffer.capacity() as _,
        }
        .send_async();

        match promise.wait() {
            Ok(resp) => match resp {
                GetProcessFieldResponse::NtPath(length) => {
                    if length != bytes {
                        // warn?
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
    /// - [PluginPermissions::PROCESS_EXECUTIVE]
    ///
    /// ## Returns
    /// - [Result] with most likely an NT error.
    ///
    /// ## Example
    /// ```rust
    ///  match process.kill_async(0).wait() {
    //         Ok(_) => {
    //             println!("Killed process!");
    //         }
    //         Err(e) => {
    //             println!("Error killing process: {:?}", e);
    //         }
    //     }
    /// ```
    pub fn kill_async(self, exit_code: u32) -> Box<AsyncPromise<EmptyResponse>> {
        KillProcessRequest {
            id: self.id,
            exit_code,
        }
        .send_async()
    }
}
