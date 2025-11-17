use crate::error::HypervisorError;
use crate::hxposed::requests::process::{CloseProcessRequest, OpenProcessRequest, ProcessOpenType};
use crate::hxposed::requests::Vmcall;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct HxProcess {
    pub id: u32,
    addr: AtomicU64,
}

impl Drop for HxProcess {
    fn drop(&mut self) {
        let _ = CloseProcessRequest {
            addr: self.addr.load(Ordering::Relaxed),
            open_type: ProcessOpenType::Hypervisor
        }.send();
    }
}

impl HxProcess {
    ///
    /// # Open
    ///
    /// Opens a process.
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
}
