use crate::error::HypervisorError;
use crate::hxposed::call::ServiceParameter;
use crate::hxposed::requests::Vmcall;
use crate::hxposed::requests::notify::*;
use crate::hxposed::responses::HypervisorResponse;
use crate::hxposed::responses::notify::AwaitNotificationResponse;
use crate::hxposed::{CallbackObject, ObjectType};
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::arch::asm;
use core::sync::atomic::{AtomicBool, Ordering};

pub type ObjectCallback = Box<dyn Fn(Result<AwaitNotificationResponse, HypervisorError>)>;

pub struct HxCallback {
    callback: CallbackObject,
    func: ObjectCallback,
    pub active: AtomicBool,
    pub target: ObjectType,
}

unsafe impl Sync for HxCallback {}
unsafe impl Send for HxCallback {}

impl Drop for HxCallback {
    fn drop(&mut self) {
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
    /// - `callback` - [`ObjectCallback`] that will be called when an event occurs.
    /// - `target` - Type of objects that will be intercepted. Valid values are:
    /// 1. [`ObjectType::Process`]
    /// 2. [`ObjectType::Thread`]
    /// 3. [`ObjectType::Registry`]. Though, not yet available
    ///
    /// ## Return
    /// * [`HxCallback`] - An abstraction that represents the callback object. The callback is active upon return.
    /// * [`HypervisorResponse::invalid_params`] with [`ServiceParameter::Arg1`] - Invalid object type specified.
    pub fn new(
        callback: ObjectCallback,
        target: ObjectType,
    ) -> Result<HxCallback, HypervisorError> {
        match target {
            ObjectType::Process(_) => {}
            ObjectType::Thread(_) => {}
            _ => {
                return Err(HypervisorError::from_response(
                    HypervisorResponse::invalid_params(ServiceParameter::Arg1),
                ));
            }
        }

        let response = RegisterNotifyHandlerRequest {
            target_object: target,
        }
        .send()?;

        Ok(Self {
            callback: response.callback,
            func: callback,
            active: AtomicBool::new(true),
            target,
        })
    }

    ///
    /// # Event Loop
    ///
    /// Main logic of the callback. Must be started on a separate task.
    pub async fn event_loop(&self) {
        unsafe{
            asm!("int 0x3")
        }

        loop {
            if !self.active.load(Ordering::Relaxed) {
                return;
            }

            match (AwaitNotificationRequest {
                callback: self.callback,
            }
            .send_async()
            .await)
            {
                Ok(x) => (self.func)(Ok(x)),
                Err(err) => {
                    (self.func)(Err(err));
                    return;
                }
            };
        }
    }
}
