use crate::utils::danger::DangerPtr;
use crate::win::unicode_string::UnicodeString;
use crate::win::{
    HANDLE, LDR_DATA_TABLE_ENTRY, OBJECT_ATTRIBUTES, ObjectAttributes, PVOID, PsLoadedModuleList,
    UNICODE_STRING,
};
use alloc::boxed::Box;
use alloc::string::String;
use core::ffi::c_void;

#[inline]
pub fn init_object_attributes(
    n: *mut UNICODE_STRING,
    a: ObjectAttributes,
    r: HANDLE,
    s: PVOID,
) -> OBJECT_ATTRIBUTES {
    OBJECT_ATTRIBUTES {
        Length: size_of::<OBJECT_ATTRIBUTES>() as _,
        RootDirectory: r,
        Attributes: a,
        ObjectName: n,
        SecurityDescriptor: s,
        SecurityQOS: s,
    }
}
/*
pub unsafe fn RtlBufferContainsBuffer(
    Buffer1: *const c_void,
    Buffer1Length: usize,
    Buffer2: *const c_void,
    Buffer2Length: usize,
) -> bool {
    if Buffer1Length < Buffer2Length + 1 {
        return false;
    }

    for i in 0..(Buffer1Length - Buffer2Length + 1) {
        let result = unsafe {
            RtlCompareMemory(
                (Buffer1 as *const u8).add(i) as *const c_void,
                Buffer2,
                Buffer2Length as _,
            )
        };

        if result == Buffer2Length as u64 {
            return true;
        }
    }

    false
}
*/
