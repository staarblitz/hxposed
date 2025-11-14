use crate::panic;
use ::alloc::boxed::Box;
use ::alloc::ffi::CString;
use ::alloc::string::String;
use ::alloc::vec::Vec;
use core::str::FromStr;
use wdk_sys::ntddk::{
    ExAllocatePool2, RtlAppendUnicodeStringToString, RtlCopyUnicodeString, RtlInitUTF8String,
    RtlUTF8StringToUnicodeString,
};
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

    ///
    /// # To Unicode String
    ///
    /// Allocates a new UNICODE_STRING on heap. Does weird stuff that takes null termination into consideration.
    ///
    /// ## Return
    /// [Box] containing [UNICODE_STRING].
    fn to_unicode_string(&self) -> Box<UNICODE_STRING> {
        let mut str = UTF8_STRING::default();
        let mut ustr = UNICODE_STRING::default();

        // +1 for null terminator since the self might NOT be null terminated. you would never know ;)
        let mut vec = Vec::<u8>::with_capacity(self.chars().count());

        unsafe {
            vec.set_len(self.len());
            core::ptr::copy(self.as_ptr(), vec.as_mut_ptr(), self.chars().count());
        }

        // !
        vec.push(0);

        unsafe {
            RtlInitUTF8String(&mut str, vec.as_ptr() as _);
            let _ = RtlUTF8StringToUnicodeString(&mut ustr, &mut str, TRUE as _);
        }
        Box::new(ustr)
    }
}
