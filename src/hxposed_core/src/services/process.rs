use crate::error::HypervisorError;
use crate::hxposed::requests::process::{CloseProcessRequest, OpenProcessRequest, ProcessOpenType};
use crate::hxposed::requests::Vmcall;
use core::sync::atomic::{AtomicU64, Ordering};

///
/// # KernelProcess
///
/// Base struct representing a process
pub trait KernelProcess {
    fn open(id: u32) -> Result<Self, HypervisorError>;
}


///
/// # NtProcess
///
/// Represents a process that works with a standard handle instead of hypervisor.
pub struct NtProcess {
    handle: AtomicU64,
}

impl Drop for NtProcess {
    fn drop(&mut self) {
        CloseProcessRequest {
            open_type: ProcessOpenType::Handle,
            addr: self.handle.load(Ordering::Relaxed)
        }.send().unwrap();
    }
}

impl KernelProcess for NtProcess {

    ///
    /// # Open
    ///
    /// Opens a kernel handle with all rights to specific process.
    ///
    /// ## Arguments
    /// id - Process id
    ///
    /// ## Returns
    /// [Result] containing [NtProcess] or error.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let process = NtProcess::open(4).unwrap();
    /// println!("{}", process.nt_path);
    /// ```
    fn open(id: u32) -> Result<Self, HypervisorError> {
        let call = OpenProcessRequest {
            process_id: id,
            open_type: ProcessOpenType::Handle
        }.send()?;

        Ok(Self{
            handle: AtomicU64::new(call.addr)
        })
    }
}