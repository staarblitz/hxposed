use wdk_sys::ntddk::{KeStackAttachProcess, KeUnstackDetachProcess};
use wdk_sys::{KAPC_STATE, PEPROCESS};

pub struct ApcProcessContext {
    apc_state: KAPC_STATE,
}

impl ApcProcessContext {
    pub fn begin(process: PEPROCESS) -> Self {
        log::trace!("Begin process APC context {:x}", process as u64);

        let mut result = Self {
            apc_state: Default::default(),
        };
        unsafe {
            KeStackAttachProcess(process, &mut result.apc_state);
        }

        result
    }
}

impl Drop for ApcProcessContext {
    fn drop(&mut self) {
        log::trace!(
            "End process APC context {:x}",
            self.apc_state.Process as u64
        );
        unsafe {
            KeUnstackDetachProcess(&mut self.apc_state);
        }
    }
}
