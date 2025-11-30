use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::{
    CloseProcessRequest, KillProcessRequest, OpenProcessRequest, ProcessOpenType,
};
use crate::hxposed::responses::empty::EmptyResponse;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::async_service::{
    AsyncPromise, GLOBAL_ASYNC_NOTIFY_HANDLER,
};
use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};

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
    /// println!("{}", process.nt_path);
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
    pub fn kill_async(self, exit_code: u32) -> AsyncPromise<EmptyResponse> {
        KillProcessRequest {
            id: self.id,
            exit_code,
        }
        .send_async()
    }
}
