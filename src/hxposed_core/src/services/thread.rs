use crate::error::HypervisorError;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::process::ObjectOpenType;
use crate::hxposed::requests::thread::*;
use crate::hxposed::responses::thread::GetThreadFieldResponse;
use crate::intern::win::GetCurrentThreadId;
use crate::plugins::plugin_perms::PluginPermissions;
use crate::services::security::HxToken;
use alloc::boxed::Box;
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
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
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
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
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
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
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
            id: self.id,
            field: ThreadField::AdjustedClientToken,

            ..Default::default()
        })
        .send_async()
        .await?
        {
            GetThreadFieldResponse::AdjustedClientToken(x) => {
                Ok(HxToken::from_raw_object(x).await?)
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
            id: self.id,
            field: ThreadField::ActiveImpersonationInfo,

            ..Default::default()
        }
        .send()?)
        {
            GetThreadFieldResponse::ActiveImpersonationInfo(x) => Ok(x),
            _ => unreachable!(),
        }
    }

    /* ///
    /// # Get Context (New)
    ///
    /// - Gets the thread context.
    /// - [`Amd64Context`] is allocated by callee.
    ///
    /// ## Warning
    /// - Make sure you have `await`ed the call to [`Self::suspend`] before calling this function.
    /// - Unlike any other function which returns number of bytes required when given a null pointer,
    /// this function does **not**. The `CONTEXT` structure has a fixed size.
    ///
    /// ## Return
    /// * [`Amd64Context`] - Thread context.
    /// * Or error
    async fn get_context_new(&mut self) -> Result<Box<Amd64Context>, HypervisorError> {
        // total heap allocation. using Box::new allocates on stack first, then moves to heap.
        let mut boxed = unsafe { Box::<Amd64Context>::new_zeroed().assume_init() };

        self.get_context(&mut boxed).await?;

        Ok(boxed)
    }

    ///
    /// # Get Context
    ///
    /// - Gets the thread context.
    /// - [`Amd64Context`] is allocated by caller.
    ///
    /// ## Warning
    /// - Make sure you have `await`ed the call to [`Self::suspend`] before calling this function.
    /// - Unlike any other function which returns number of bytes required when given a null pointer,
    /// this function does **not**. The `CONTEXT` structure has a fixed size.
    ///
    /// ## Return
    /// * [`Amd64Context`] - Thread context.
    /// * Or error
    async fn get_context(
        &mut self,
        ctx: &mut Box<Amd64Context>,
    ) -> Result<(), HypervisorError> {
        GetSetThreadContextRequest {
            id: self.id,
            operation: ThreadContextOperation::Get,
            data: ctx.as_mut() as *mut _ as _,
            data_len: size_of::<Amd64Context>(),
        }
        .send_async()
        .await?;

        Ok(())
    }

    ///
    /// # Get Context
    ///
    /// Sets the thread context.
    ///
    /// ## Arguments
    ///
    /// ## Warning
    /// - Make sure you have `await`ed the call to [`Self::suspend`] before calling this function.
    /// - Unlike any other function which returns number of bytes required when given a null pointer,
    /// this function does **not**. The `CONTEXT` structure has a fixed size.
    ///
    /// ## Return
    /// * Error or ().
    async fn set_context(&mut self, mut ctx: Box<Amd64Context>) -> Result<(), HypervisorError> {
        GetSetThreadContextRequest {
            id: self.id,
            operation: ThreadContextOperation::Set,
            data: ctx.as_mut() as *mut _ as _,
            data_len: size_of::<Amd64Context>(),
        }
        .send_async()
        .await?;

        Ok(())
    }*/

    ///
    /// # Kill
    ///
    /// Terminates current thread.
    ///
    /// ## Permissions
    /// * [`PluginPermissions::THREAD_EXECUTIVE`]
    ///
    /// ## Warning
    /// Note that this is not an OP command. The thread will get stuck if it's waiting for an I/O operation.
    pub async fn kill(&mut self, exit_code: u32) -> Result<(), HypervisorError> {
        KillThreadRequest {
            id: self.id,
            exit_code,
        }
        .send_async()
        .await?;

        Ok(())
    }

    ///
    /// # Freeze
    ///
    /// Freezes the specified thread. Not Implemented
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
