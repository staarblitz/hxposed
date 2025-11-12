use ::alloc::boxed::Box;
use ::alloc::string::String;
use ::alloc::vec::Vec;
use wdk_sys::{HANDLE, NTSTATUS, OBJECT_ATTRIBUTES, PCSZ, POBJECT_ATTRIBUTES, PROCESSINFOCLASS, PULONG, PUNICODE_STRING, PVOID, TRUE, ULONG, UNICODE_STRING, UTF8_STRING};
use wdk_sys::ntddk::{RtlInitUTF8String, RtlUTF8StringToUnicodeString};

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
#[allow(non_snake_case)]
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

#[inline]
pub unsafe fn to_utf16(s: &str) -> Box<UNICODE_STRING> {
    let mut str = UTF8_STRING::default();
    let mut ustr = UNICODE_STRING::default();
    RtlInitUTF8String(&mut str, s.as_ptr() as _);
    let _ = RtlUTF8StringToUnicodeString(&mut ustr, &mut str, TRUE as _);

    Box::new(ustr)
}
