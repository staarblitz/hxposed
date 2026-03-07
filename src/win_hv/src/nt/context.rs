use crate::win::{KAPC_STATE, KeStackAttachProcess, KeUnstackDetachProcess, PEPROCESS};

pub struct ApcProcessContext {
    apc_state: KAPC_STATE,
}

impl ApcProcessContext {
    pub fn begin(process: PEPROCESS) -> Self {
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
        unsafe {
            KeUnstackDetachProcess(&mut self.apc_state);
        }
    }
}
