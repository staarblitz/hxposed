use core::mem;
use wdk_sys::HANDLE;
use wdk_sys::ntddk::ZwClose;

pub struct HandleBox {
    handle: HANDLE,
}

impl HandleBox {
    pub fn new(handle: HANDLE) -> HandleBox {
        Self { handle }
    }

    ///
    /// # Get Danger
    ///
    /// Gets the handle object. (Copy)
    pub fn get_danger(&self) -> HANDLE {
        self.handle
    }

    ///
    /// # Get Forget
    ///
    /// Returns the handle, consumes the object, but does NOT close it.
    pub fn get_forget(self) -> HANDLE {
        let handle = self.handle;
        mem::forget(self);

        handle
    }
}

impl Drop for HandleBox {
    fn drop(&mut self) {
        let _ = unsafe { ZwClose(self.handle) };
    }
}
