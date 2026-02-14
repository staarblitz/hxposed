use crate::pe::HOOK_BYTES;
use crate::utils;
use core::ptr::{copy_nonoverlapping, null};
use uefi::boot::MemoryAttribute;
use uefi::table::cfg::MemoryProtectionAttribute;

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
        log::trace!(
            "Initializing Detour. Hook addr: 0x{:x}, detour addr: 0x{:x})",
            hook_addr.addr(),
            detour_addr.addr()
        );
        self.detour_address = detour_addr;
        self.original_address = hook_addr;
    }

    pub fn detour(&mut self) {
        // steal the bytes
        log::trace!(
            "Stealing {} bytes from 0x{:x} into 0x{:x}",
            self.original_bytes.len(),
            self.original_address.addr(),
            self.original_bytes.as_ptr().addr()
        );
        unsafe {
            copy_nonoverlapping(
                self.original_address as *mut u8,
                self.original_bytes.as_mut_ptr(),
                self.original_bytes.len(),
            );
        }

        log::trace!("Bytes stolen: {:x?}", &self.original_bytes,);

        // place the hook
        log::trace!(
            "Placing {} bytes from 0x{:x} into 0x{:x}",
            HOOK_BYTES.len(),
            HOOK_BYTES.as_ptr().addr(),
            self.original_address.addr(),
        );
        unsafe {
            // seems like behavior have changed since recent bootmgfw update. the pages are not RW.
            // which means we have to do it ourselves

            copy_nonoverlapping(
                HOOK_BYTES.as_ptr(),
                self.original_address as _,
                HOOK_BYTES.len(),
            );

            // write the address to jump to
            let addr = self.original_address.byte_offset(2) as *mut u64;
            log::trace!(
                "Writing detour address 0x{:x} into 0x{:x}",
                self.original_address.byte_offset(2).addr(),
                addr.addr()
            );
            addr.write(self.detour_address as _);
        }
        log::trace!("Hooking complete");
        self.is_hooked = true;
    }

    pub fn revert(&mut self) {
        // return stolen bytes
        log::trace!(
            "Returning stolen {:x?} into 0x{:x}",
            self.original_bytes,
            self.original_address.addr(),
        );
        unsafe {
            copy_nonoverlapping(
                self.original_bytes.as_mut_ptr(),
                self.original_address as *mut u8,
                self.original_bytes.len(),
            );
        }

        log::trace!("Unhooked");

        self.is_hooked = false;
    }
}
