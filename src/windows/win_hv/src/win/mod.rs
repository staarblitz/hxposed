use crate::as_utf16;
use ::alloc::vec::Vec;
use core::iter::once;
use core::ptr::null_mut;
use utf16string::{WStr, WString};
use wdk_sys::{
    HANDLE, NTSTATUS, OBJECT_ATTRIBUTES, POBJECT_ATTRIBUTES, PROCESSINFOCLASS, PULONG,
    PUNICODE_STRING, PVOID, ULONG,
};

pub(crate) mod alloc;
pub(crate) mod macros;

#[link(name = "ntoskrnl")]
unsafe extern "C" {
    #[allow(non_snake_case)]
    pub fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}

#[inline]
pub unsafe fn InitializeObjectAttributes(
    p: POBJECT_ATTRIBUTES,
    n: &str,
    a: ULONG,
    r: HANDLE,
    s: PVOID,
) {
    use core::mem::size_of;
    unsafe {
        (*p).Length = size_of::<OBJECT_ATTRIBUTES>() as ULONG;
        (*p).RootDirectory = r;
        (*p).Attributes = a;
        (*p).ObjectName = WString::from(n).as_mut_wstr().as_mut();
        (*p).SecurityDescriptor = s;
        (*p).SecurityQualityOfService = s;
    }
}
