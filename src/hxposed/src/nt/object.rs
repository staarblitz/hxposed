#![allow(unsafe_op_in_unsafe_fn)]

use crate::nt::{get_object_body, get_object_header, ObjectBody, ObjectHeader};
use crate::utils::intrin::{interlocked_decrement, interlocked_increment};
use crate::win::HANDLE;
use crate::win::{ExpLookupHandleTableEntry, PHANDLE_TABLE, _EXHANDLE};
use bit_field::BitField;
use core::ffi::c_void;
use hxposed_core::hxposed::Handle;

/// This is not a trait, nor an abstraction layer. This is for general object functions.
pub struct NtObject {
    pub object_addr: ObjectBody,
}

pub struct NtHandle;

pub type HandleTableEntry = *mut u64;

impl Drop for NtObject {
    fn drop(&mut self) {
        unsafe {
            Self::decrement_ref_count(get_object_header(self.object_addr as _) as _);
        }
    }
}

impl NtObject {
    pub fn from_ptr(ptr: ObjectBody) -> Self {
        unsafe {
            Self::increment_ref_count(get_object_header(ptr));
        }
        Self { object_addr: ptr }
    }

    pub unsafe fn increment_ref_count(obj_header: ObjectHeader) {
        interlocked_increment(obj_header.0);
    }

    pub unsafe fn decrement_ref_count(obj_header: ObjectHeader) {
        interlocked_decrement(obj_header.0);
    }

    pub unsafe fn increment_ref_count_raw(obj_body: *mut c_void) {
        Self::increment_ref_count(get_object_header(ObjectBody(obj_body as _)));
    }

    pub unsafe fn decrement_ref_count_raw(obj_body: *mut c_void) {
        Self::decrement_ref_count(get_object_header(ObjectBody(obj_body as _)));
    }

    pub unsafe fn increment_handle_count(obj_header: ObjectHeader) {
        interlocked_increment(obj_header.0.byte_offset(8));
    }

    pub unsafe fn decrement_handle_count(obj_header: ObjectHeader) {
        interlocked_decrement(obj_header.0.byte_offset(8));
    }

    pub unsafe fn increment_handle_count_raw(obj_body: *mut c_void) {
        Self::increment_handle_count(get_object_header(ObjectBody(obj_body as _)));
    }

    pub unsafe fn decrement_handle_count_raw(obj_body: *mut c_void) {
        Self::decrement_handle_count(get_object_header(ObjectBody(obj_body as _)));
    }

    pub fn from_handle_entry(entry: HandleTableEntry) -> NtObject {
        NtObject::from_ptr(NtHandle::get_object_ptr(entry))
    }

    pub fn from_handle(handle: HANDLE, table: PHANDLE_TABLE) -> Option<NtObject> {
        let entry = NtHandle::get_handle_entry(handle as _, table)?;
        Some(Self::from_handle_entry(entry))
    }

    // cannot be used due to occurring apc issues. deprecated
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
    pub fn get_object_ptr(entry: HandleTableEntry) -> ObjectBody {
        let object_pointer = unsafe { *entry }.get_bits(20..64);
        let object_header = (object_pointer << 4 | 0xffff000000000000) as *mut c_void; // decode bitmask to get real ptr

        // object body is always after object header. so we add sizeof(OBJECT_HEADER) (and -8) which is 0x30 to get object itself
        unsafe { get_object_body(ObjectHeader(object_header as _)) }
    }

    pub fn get_granted_access(entry: HandleTableEntry) -> u32 {
        unsafe { entry.add(1).read_unaligned() }.get_bits(0..25) as _
    }

    pub fn set_object_ptr(entry: HandleTableEntry, ptr: ObjectBody) {
        let header = unsafe { get_object_header(ptr as _) };
        let old_object = unsafe { get_object_header(Self::get_object_ptr(entry) as _) };
        unsafe {
            NtObject::increment_ref_count(header);

            // no. we cannot simply increment or decrement handle count. we have to invoke OpenProcedure and CloseProcedure
            // like the object manager does. or we fuck up. like i currently do
            NtObject::increment_handle_count(header);

            // then dereference the actual object this handle points to
            NtObject::decrement_handle_count(old_object);
            NtObject::decrement_ref_count(old_object);
        }

        let compressed = ((header.0 as u64) & 0x0000ffffffffffff) >> 4;
        let mut old = unsafe { *entry }; // copy the value locally
        let new_value = old.set_bits(20..64, compressed);
        unsafe {
            entry.write_unaligned(*new_value) // write to it
        }
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
