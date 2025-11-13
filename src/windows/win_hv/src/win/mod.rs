use crate::panic;
use ::alloc::boxed::Box;
use ::alloc::string::String;
use ::alloc::vec::Vec;
use wdk_sys::ntddk::{ExAllocatePool2, RtlAppendUnicodeStringToString, RtlCopyUnicodeString, RtlInitUTF8String, RtlUTF8StringToUnicodeString};
use wdk_sys::{
    HANDLE, NTSTATUS, OBJECT_ATTRIBUTES, PCSZ, POBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED,
    PROCESSINFOCLASS, PULONG, PUNICODE_STRING, PVOID, TRUE, ULONG, UNICODE_STRING, UTF8_STRING,
};

pub(crate) mod alloc;
pub(crate) mod macros;
pub(crate) mod timing;

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

#[allow(non_snake_case)]
pub unsafe fn _RtlDuplicateUnicodeString(
    first: &mut UNICODE_STRING,
    second: &mut UNICODE_STRING,
) -> Box<UNICODE_STRING> {
    let mut result = UNICODE_STRING::default();

    result.Buffer = unsafe {
        ExAllocatePool2(
            POOL_FLAG_NON_PAGED,
            (first.MaximumLength + second.MaximumLength) as _,
            0xFFF,
        )
    } as _;

    if result.Buffer.is_null() {
        panic!("Failed to allocate unicode string");
    }

    result.MaximumLength = first.MaximumLength + second.MaximumLength;
    result.Length = first.Length + second.Length;

    unsafe {
        RtlCopyUnicodeString(&mut result, first);
        RtlAppendUnicodeStringToString(&mut result, second);
    }

    Box::new(result)
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

pub trait Utf8ToUnicodeString {
    fn to_unicode_string(&self) -> Box<UNICODE_STRING>;
}

impl Utf8ToUnicodeString for str {
    fn to_unicode_string(&self) -> Box<UNICODE_STRING> {
        let mut str = UTF8_STRING::default();
        let mut ustr = UNICODE_STRING::default();
        unsafe {
            RtlInitUTF8String(&mut str, self.as_ptr() as _);
            let _ = RtlUTF8StringToUnicodeString(&mut ustr, &mut str, TRUE as _);
        }
        Box::new(ustr)
    }
}
