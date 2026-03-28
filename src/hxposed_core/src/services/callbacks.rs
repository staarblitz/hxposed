use crate::error::HxError;
use crate::hxposed::requests::Syscall;
use crate::hxposed::requests::notify::*;
use crate::hxposed::responses::notify::*;
use crate::hxposed::{CallbackObject, ObjectType};
use crate::intern::win::{CloseHandle, CreateEventA, ResetEvent, SetEvent, WaitForSingleObject};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, Ordering};

pub struct HxCallback {
    callback: CallbackObject,
    pub active: AtomicBool,
    pub target: ObjectType,
    pub event_handle: u64,
    pub response_buffer: Box<CallbackInformation>,
}

unsafe impl Sync for HxCallback {}
unsafe impl Send for HxCallback {}

impl Drop for HxCallback {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.event_handle);
        }
        self.active.store(false, Ordering::SeqCst);
        UnregisterNotifyHandlerRequest {
            callback: self.callback,
        }
        .send()
        .unwrap();
    }
}

impl HxCallback {
    ///
    /// # New
    ///
    /// Creates a new kernel callback for specified object type.
    ///
    /// To deregister the callback, use [`drop`].
    ///
    /// ## Arguments
    /// - `target` - Type of objects that will be intercepted. Valid values are:
    /// 1. [`ObjectType::Process(0)`]
    /// 2. [`ObjectType::Thread(0)`]
    /// 3. [`ObjectType::Registry(0)`]. Though, not yet available
    ///
    /// ## Return
    /// * [`HxCallback`] - An abstraction that represents the callback object. The callback is active upon return.
    /// * [`SyscallResponseInfo::invalid_params`] with [`ServiceParameter::Arg1`] - Invalid object type specified.
    pub fn new(target: ObjectType) -> Result<HxCallback, HxError> {
        match target {
            ObjectType::Process(_) => {}
            ObjectType::Thread(_) => {}
            _ => {
                return Err(HxError::InvalidParameters(0));
            }
        }

        let event_handle = unsafe { CreateEventA(null_mut(), 0, 0, null_mut()) };

        let buffer = Box::<CallbackInformation>::new(CallbackInformation::default());

        let response = RegisterNotifyHandlerRequest {
            target_object: target,
            event_handle,
            memory: buffer.as_ref() as *const _ as _,
        }
        .send()?;

        Ok(Self {
            callback: response.callback,
            active: AtomicBool::new(true),
            target,
            event_handle,
            response_buffer: buffer,
        })
    }

    ///
    /// # Wait For Callback
    ///
    /// Waits for hypervisor to signal the event.
    ///
    /// ## Return
    /// * [`CallbackInformation`] - Information about the callback.
    /// * [`HxError`] - Timed out.
    ///
    /// ## Example
    /// ```rust
    /// let callback = HxCallback::new(ObjectType::Process(0)).unwrap();
    /// loop {
    ///     match callback.wait_for_callback() {
    ///         Ok(info) => {
    ///             let process = HxProcess::open(info.object_value)
    ///         }
    ///         Err(_) => /* ignore */
    ///     }
    /// }
    /// ```
    pub fn wait_for_callback(&self) -> Result<CallbackInformation, HxError> {
        match unsafe { WaitForSingleObject(self.event_handle, 2000) } {
            0 => Ok(self.response_buffer.as_ref().clone()),
            _ => Err(HxError::TimedOut),
        }
    }
}
