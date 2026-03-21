#![allow(unsafe_op_in_unsafe_fn)]

use crate::nt::{get_object_body, get_object_header};
use crate::utils::intrin::{interlocked_decrement, interlocked_increment};
use crate::win::HANDLE;
use crate::win::{ExpLookupHandleTableEntry, PHANDLE_TABLE, _EXHANDLE};
use bit_field::BitField;
use core::ffi::c_void;
use hxposed_core::hxposed::Handle;

/// This is not a trait, nor an abstraction layer. This is for general object functions.
#[derive(Debug)]
pub struct NtObject<T> {
    pub object_addr: *mut T,
}

pub struct NtHandle;

pub type HandleTableEntry = *mut u64;

impl<T> Drop for NtObject<T> {
    fn drop(&mut self) {
        unsafe {
            Self::decrement_ref_count(self.object_addr as _);
        }
    }
}

impl<T> NtObject<T> {
    pub fn from_ptr(ptr: *mut T) -> Self {
        unsafe {
            Self::increment_ref_count(ptr as _);
        }
        Self { object_addr: ptr }
    }

    pub unsafe fn increment_ref_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x30) };
        interlocked_increment(header as _);
    }

    pub unsafe fn decrement_ref_count(object: *mut c_void) {
        let header = unsafe { object.byte_offset(-0x30) };
        interlocked_decrement(header as _);
    }

    pub unsafe fn increment_handle_count(object: *mut T) {
        let header = unsafe { object.byte_offset(-0x28) };
        interlocked_increment(header as _);
    }

    pub unsafe fn decrement_handle_count(object: *mut T) {
        let header = unsafe { object.byte_offset(-0x28) };
        interlocked_decrement(header as _);
    }

    pub fn from_handle_entry(entry: HandleTableEntry) -> NtObject<T> {
        NtObject::from_ptr(NtHandle::get_object_ptr(entry))
    }

    pub fn from_handle(handle: HANDLE, table: PHANDLE_TABLE) -> Option<NtObject<T>> {
        let entry = NtHandle::get_handle_entry(handle as _, table)?;
        Some(Self::from_handle_entry(entry))
    }

    // cannot be used due to occuring apc issues
    /*pub fn create_handle(object: *mut T, table: PHANDLE_TABLE) -> Result<HANDLE, ()> {
        let handle = unsafe { ExCreateHandle(table, get_object_header(object as _) as _) };

        let exhandle = _EXHANDLE { Value: handle as _ };

        // so we need to look up the entry again, because ExCreateHandle grants 0 accesses.
        let entry = unsafe{&mut*Self::get_handle_entry(exhandle, table).unwrap()};
        Self::upgrade_handle(entry, Self::HANDLE_ALL_ACCESS)?;

        let object_pointer = entry.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut c_void;
        let object = unsafe { get_object_body(object_header) };

        // we have to increment BOTH handle and pointer count since ExCreateHandle does none of that
        unsafe {
            Self::increment_handle_count(object);
            Self::increment_ref_count(object);
        }

        Ok(handle as _)
    }*/
}

impl NtHandle {
    pub fn get_object_ptr<T>(entry: HandleTableEntry) -> *mut T {
        let object_pointer = unsafe { *entry }.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut c_void; // decode bitmask to get real ptr

        // object body is always after object header. so we add sizeof(OBJECT_HEADER) (and -8) which is 0x30 to get object itself
        (unsafe { get_object_body(object_header) } as *mut T)
    }

    pub fn get_granted_access(entry: HandleTableEntry) -> u32 {
        unsafe { entry.add(1).read_unaligned() }.get_bits(0..25) as _
    }

    pub fn set_object_ptr<T>(entry: HandleTableEntry, ptr: *mut T) {
        let header = unsafe { get_object_header(ptr as _) };
        let compressed = ((header as u64) & 0x0000ffffffffffff) >> 4;
        unsafe { *entry }.set_bits(20..64, compressed);
    }

    pub fn upgrade_handle(entry: HandleTableEntry, access_mask: u32) {
        // +1 since the offset of GrantedAccessRights is 0x8
        let mut old = unsafe { entry.add(1).read_unaligned() }; // read_unaligned copies
        let new_value = old.set_bits(0..25, access_mask as u64); // otherwise we would get an unaligned access
        unsafe {
            entry.add(1).write_unaligned(*new_value);
        }
    }

    pub fn get_handle_entry(handle: Handle, table: PHANDLE_TABLE) -> Option<HandleTableEntry> {
        let entry = unsafe { ExpLookupHandleTableEntry(table, _EXHANDLE { Value: handle }) };
        if entry.is_null() { None } else { Some(entry) }
    }
}
