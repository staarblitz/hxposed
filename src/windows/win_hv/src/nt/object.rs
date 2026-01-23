use bit_field::BitField;
use wdk_sys::{HANDLE};

use crate::{
    win::{_EXHANDLE, ExpLookupHandleTableEntry, PHANDLE_TABLE},
};
use crate::nt::{get_object_body, get_object_header};
use crate::win::ExCreateHandle;

/// This is not a trait, nor an abstraction layer. This is for general object functions.
#[derive(Debug)]
pub struct NtObject<T> {
    pub object_addr: *mut T
}

impl<T> Drop for NtObject<T> {
    fn drop(&mut self) {
        unsafe {
            Self::decrement_ref_count(self.object_addr as _);
        }
    }
}

impl<T> NtObject<T> {
    pub const LOW_LEVEL_ENTRIES: u64 = 4096 / 0x80;

    pub fn from_ptr(ptr: *mut T) -> Self {
        unsafe {
            Self::increment_ref_count(ptr as _);
        }
        Self {
            object_addr: ptr
        }
    }

    // maybe we should make this atomic?
    // meh, we are in a VMEXIT anyway
    pub unsafe fn increment_ref_count(object: *mut u64) {
        let header = unsafe{object.offset(-0x30)};
        unsafe{header.write(*header +1)};
    }

    pub unsafe fn decrement_ref_count(object: *mut u64) {
        let header = unsafe{object.offset(-0x30)};
        unsafe{header.write(*header -1)};
    }

    pub unsafe fn increment_handle_count(object: *mut u64) {
        let header = unsafe{object.offset(-0x28)};
        unsafe{header.write(*header +1)};
    }

    pub unsafe fn decrement_handle_count(object: *mut u64) {
        let header = unsafe{object.offset(-0x28)};
        unsafe{header.write(*header -1)};
    }

    pub fn from_handle(handle: HANDLE, table: PHANDLE_TABLE) -> Result<NtObject<T>, ()> {
        let handle = handle as u64;
        let exhandle = _EXHANDLE {
            Value: handle
        };
        let handle_table_entry = unsafe{ExpLookupHandleTableEntry(table, exhandle)};
        if handle_table_entry.is_null() {
            return Err(());
        }

        let object_pointer = unsafe{*(handle_table_entry)}.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut u64; // decode bitmask to get real ptr

        // object body is always after object header. so we add sizeof(OBJECT_HEADER) which is 0x30 to get object itself
        let object_body = unsafe{get_object_body(object_header)} as *mut T;

        // increment ref count manually so when handle is closed, the object won't be dropped.

        Ok(Self::from_ptr(object_body))
    }

    pub fn create_handle(object: *mut T, table: PHANDLE_TABLE) -> Result<HANDLE, ()> {
        let handle = unsafe {
            ExCreateHandle(table, get_object_header(object as _) as _)
        };

        let exhandle = _EXHANDLE {
            Value: handle as _
        };

        // so we need to look up the entry again, because ExCreateHandle grants 0 accesses.
        let entry = unsafe {
            ExpLookupHandleTableEntry(table, exhandle)
        };
        if entry.is_null() {
            return Err(())
        }

        let entry = unsafe{&mut *entry};

        // give all access
        entry.set_bits(0..25, 0x1FFFFFF);

        let object_pointer = entry.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut u64;
        let object = unsafe{get_object_body(object_header)};

        // we have to increment BOTH handle and pointer count since ExCreateHandle does none of that
        unsafe {
            Self::increment_handle_count(object);
            Self::increment_ref_count(object);
        }

        Ok(handle as _)
    }
}
