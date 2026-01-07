use alloc::boxed::Box;
use core::ffi::c_void;
use wdk_sys::{FALSE, HANDLE, LIST_ENTRY, OBJECT_ATTRIBUTES, POBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED, PUNICODE_STRING, PVOID, ULONG, UNICODE_STRING};
use wdk_sys::ntddk::{ExAllocatePool2, RtlCompareMemory, RtlCompareUnicodeString, RtlCopyUnicodeString};
use crate::utils::danger::DangerPtr;
use crate::win::{PsLoadedModuleList, _LDR_DATA_TABLE_ENTRY};
use crate::win::utf_to_unicode::Utf8ToUnicodeString;

pub unsafe fn RtlGetLoadedModuleByName(name: &str) -> Option<*mut _LDR_DATA_TABLE_ENTRY> { unsafe {
    let mut unicoded = name.to_unicode_string();
    let list = DangerPtr {
        ptr: PsLoadedModuleList,
    };

    let head = &list.InLoadOrderLinks as *const LIST_ENTRY;
    let mut current = (*head).Flink;

    let mut return_value: Option<*mut _LDR_DATA_TABLE_ENTRY> = None;

    while current.addr() != head.addr() {
        let entry = &mut *(current as *mut _LDR_DATA_TABLE_ENTRY);

        match RtlCompareUnicodeString(&entry.BaseDllName, unicoded.as_mut(), FALSE as _) {
            0 => {
                return_value = Some(current as *mut _LDR_DATA_TABLE_ENTRY);
                break;
            },
            _ => {}
        }

        current = (*current).Flink;
    }

    return_value
}}

pub unsafe fn _RtlDuplicateUnicodeString(
    first: &mut UNICODE_STRING,
    length: u16,
) -> Box<UNICODE_STRING> {
    let mut result = UNICODE_STRING::default();

    result.Buffer = unsafe {
        ExAllocatePool2(
            POOL_FLAG_NON_PAGED,
            (first.MaximumLength + length) as _,
            0xFFF,
        )
    } as _;

    if result.Buffer.is_null() {
        panic!("Failed to allocate unicode string");
    }

    result.MaximumLength = first.MaximumLength + length;
    result.Length = first.Length + length;

    unsafe {
        RtlCopyUnicodeString(&mut result, first);
    }

    Box::new(result)
}

#[inline]
pub unsafe fn InitializeObjectAttributes(
    p: POBJECT_ATTRIBUTES,
    n: PUNICODE_STRING,
    a: ULONG,
    r: HANDLE,
    s: PVOID,
) {
    use core::mem::size_of;
    unsafe {
        (*p).Length = size_of::<OBJECT_ATTRIBUTES>() as ULONG;
        (*p).RootDirectory = r;
        (*p).Attributes = a;
        (*p).ObjectName = n;
        (*p).SecurityDescriptor = s;
        (*p).SecurityQualityOfService = s;
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