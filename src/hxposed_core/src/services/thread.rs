use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::thread::*;
use crate::intern::win::GetCurrentThreadId;
use crate::plugins::plugin_perms::PluginPermissions;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct HxThread {
    pub id: u32,
    pub addr: AtomicU64,
    process_id: u32,
}

impl Drop for HxThread {
    fn drop(&mut self) {
        let _ = CloseThreadRequest {
            addr: self.addr.load(Ordering::Relaxed),
            open_type: ObjectOpenType::Hypervisor,
        }
        .send();
    }
}

impl HxThread {
    ///
    /// # Current
    ///
    /// Opens the current thread for your use.
    pub fn current() -> Result<Self, HypervisorError> {
        Self::open(unsafe { GetCurrentThreadId() })
    }

    ///
    /// # Suspend
    ///
    /// Suspends the specified thread.
    ///
    /// ## Return
    /// * [`u32`] - Previous suspend count
    pub async fn suspend(&mut self) -> Result<u32, HypervisorError> {
        let result = SuspendResumeThreadRequest {
            id: self.id,
            operation: SuspendResumeThreadOperation::Suspend,
        }
        .send_async()
        .await?;

        Ok(result.previous_count)
    }

    ///
    /// # Resume
    ///
    /// Resumes the specified thread.
    ///
    /// ## Return
    /// * [`u32`] - Previous suspend count
    pub async fn resume(&mut self) -> Result<u32, HypervisorError> {
        let result = SuspendResumeThreadRequest {
            id: self.id,
            operation: SuspendResumeThreadOperation::Resume,
        }
            .send_async()
            .await?;

        Ok(result.previous_count)
    }

    ///
    /// # Freeze
    ///
    /// Freezes the specified thread.
    ///
    /// ## Return
    /// * [`u32`] - Previous freeze count
    async fn freeze(&mut self) -> Result<u32, HypervisorError> {
        let result = SuspendResumeThreadRequest {
            id: self.id,
            operation: SuspendResumeThreadOperation::Freeze,
        }
            .send_async()
            .await?;

        Ok(result.previous_count)
    }

    ///
    /// # Open Handle
    ///
    /// Returns a handle with `THREAD_ALL_ACCESS`.
    ///
    /// ## Arguments
    /// * `id` - Thread id
    ///
    /// ## Warning
    /// - The caller holds full ownership to the handle.
    ///
    /// ## Returns
    /// * Handle as an u64.
    pub async fn open_handle(id: u32) -> Result<u64, HypervisorError> {
        let result = OpenThreadRequest {
            pid: 0,
            tid: id,
            open_type: ObjectOpenType::Handle,
        }
        .send_async()
        .await?;

        Ok(result.addr)
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
            pid: 0,
            tid: id,
            open_type: ObjectOpenType::Hypervisor,
        }
        .send()?;

        Ok(Self {
            id,
            process_id: 0,
            addr: AtomicU64::new(result.addr),
        })
    }
}
