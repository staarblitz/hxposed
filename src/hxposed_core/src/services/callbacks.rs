use core::ptr::null_mut;
use crate::error::HypervisorError;
use crate::hxposed::call::ServiceParameter;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::notify::*;
use crate::hxposed::responses::{read_response_type, HypervisorResponse};
use crate::hxposed::responses::notify::*;
use crate::hxposed::{CallbackObject, ObjectType};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::intern::win::{CloseHandle, CreateEventA, ResetEvent, SetEvent, WaitForSingleObject};

pub struct HxCallback {
    callback: CallbackObject,
    pub active: AtomicBool,
    pub target: ObjectType,
    pub event_handle: u64,
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
    /// ## Permissions
    /// * [`PluginPermissions::INTERCEPT_PROCESS`] if `target` is [`ObjectType::Process`]
    /// * [`PluginPermissions::INTERCEPT_THREAD`] if `target` is [`ObjectType::Thread`]
    ///
    /// Remember that you may also need [`PluginPermissions::PROCESS_EXECUTIVE`] or [`PluginPermissions::THREAD_EXECUTIVE`] if you want to control those objects.
    ///
    /// ## Arguments
    /// - `target` - Type of objects that will be intercepted. Valid values are:
    /// 1. [`ObjectType::Process`]
    /// 2. [`ObjectType::Thread`]
    /// 3. [`ObjectType::Registry`]. Though, not yet available
    ///
    /// ## Return
    /// * [`HxCallback`] - An abstraction that represents the callback object. The callback is active upon return.
    /// * [`HypervisorResponse::invalid_params`] with [`ServiceParameter::Arg1`] - Invalid object type specified.
    pub fn new(target: ObjectType) -> Result<HxCallback, HypervisorError> {
        match target {
            ObjectType::Process(_) => {}
            _ => {
                return Err(HypervisorError::from_response(
                    HypervisorResponse::invalid_params(ServiceParameter::Arg1),
                ));
            }
        }

        let event_handle = unsafe {
            CreateEventA(null_mut(), 0, 0, null_mut())
        };

        let response = RegisterNotifyHandlerRequest {
            target_object: target,
            event_handle,
        }
        .send()?;

        Ok(Self {
            callback: response.callback,
            active: AtomicBool::new(true),
            target,
            event_handle
        })
    }

    ///
    /// # Wait For Callback
    ///
    /// Waits for hypervisor to signal the event.
    ///
    /// ## Return
    /// * [`CallbackInformation`] - Information about the callback.
    /// * [`HypervisorError`] - Timed out.
    ///
    /// ## Example
    /// ```rust
    /// let callback = HxCallback::new(ObjectType::Process(0)).unwrap();
    /// loop {
    ///     match callback.wait_for_callback() {
    ///         Ok(info) => {
    ///             // do something
    ///         }
    ///         Err(_) => /* ignore */
    ///     }
    /// }
    /// ```
    pub fn wait_for_callback(&self) -> Result<CallbackInformation, HypervisorError>
    {
        let response = match unsafe {
            WaitForSingleObject(self.event_handle, 2000)
        } {
            0 => Ok(unsafe {
                read_response_type::<CallbackInformation>(CALLBACK_RESPONSE_RESERVED_OFFSET)
            }),
            _ => Err(HypervisorError::async_time_out())
        };

        response
    }
}
