use crate::utils::danger::DangerPtr;
use crate::win::unicode_string::UnicodeString;
use crate::win::{_LDR_DATA_TABLE_ENTRY, PsLoadedModuleList};
use alloc::boxed::Box;
use alloc::string::String;
use core::ffi::c_void;
use wdk_sys::ntddk::{
    ExAllocatePool2, RtlCompareMemory, RtlCompareUnicodeString, RtlCopyUnicodeString,
};
use wdk_sys::{
    FALSE, HANDLE, LIST_ENTRY, OBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED, PUNICODE_STRING, PVOID,
    ULONG, UNICODE_STRING,
};

#[inline]
pub fn init_object_attributes(
    n: PUNICODE_STRING,
    a: ULONG,
    r: HANDLE,
    s: PVOID,
) -> OBJECT_ATTRIBUTES {
    OBJECT_ATTRIBUTES {
        Length: size_of::<OBJECT_ATTRIBUTES>() as _,
        RootDirectory: r,
        Attributes: a,
        ObjectName: n,
        SecurityDescriptor: s,
        SecurityQualityOfService: s,
    }
}

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
