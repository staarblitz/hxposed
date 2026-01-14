use crate::pe::HOOK_BYTES;
use core::ptr::{copy_nonoverlapping, null};

pub struct Detour<T> {
    pub original_address: *const T,
    pub detour_address: *const T,
    pub original_bytes: [u8; 12],
    pub is_hooked: bool,
}

unsafe impl<T> Send for Detour<T> {}
unsafe impl<T> Sync for Detour<T> {}

impl<T> Detour<T> {
    // the Default trait isn't constant. so we got this one instead.
    pub const fn default() -> Self {
        Self {
            original_address: null(),
            detour_address: null(),
            original_bytes: [0; 12],
            is_hooked: false,
        }
    }

    pub fn init(&mut self, hook_addr: *const T, detour_addr: *const T) {
        self.detour_address = detour_addr;
        self.original_address = hook_addr;
    }

    pub fn detour(&mut self) {
        // steal the bytes
        unsafe {
            copy_nonoverlapping(
                self.original_address as *mut u8,
                self.original_bytes.as_mut_ptr(),
                self.original_bytes.len(),
            );
        }

        // place the hook
        unsafe {
            copy_nonoverlapping(
                HOOK_BYTES.as_ptr(),
                self.original_address as _,
                HOOK_BYTES.len(),
            );

            // write the address to jump to
            let addr = self.original_address.byte_offset(2) as *mut u64;
            addr.write(self.detour_address as _);
        }

        self.is_hooked = true;
    }

    pub fn revert(&mut self) {
        // return stolen bytes
        unsafe {
            copy_nonoverlapping(
                self.original_bytes.as_mut_ptr(),
                self.original_address as *mut u8,
                self.original_bytes.len(),
            );
        }

        self.is_hooked = false;
    }
}
