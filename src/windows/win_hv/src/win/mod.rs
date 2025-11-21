use ::alloc::boxed::Box;
use ::alloc::vec::Vec;
use core::arch::asm;
use core::ffi::c_void;
use core::ptr::null_mut;
use core::str::FromStr;
use core::sync::atomic::{AtomicPtr, Ordering};
use wdk_sys::ntddk::{
    ExAllocatePool2, RtlCompareMemory, RtlCopyUnicodeString, RtlInitUTF8String,
    RtlUTF8StringToUnicodeString,
};
use wdk_sys::{
    ACCESS_MASK, BOOLEAN, HANDLE, KPROCESSOR_MODE, LIST_ENTRY, NTSTATUS, OBJECT_ATTRIBUTES,
    PCLIENT_ID, PCONTEXT, PEPROCESS, PETHREAD, PHANDLE, POBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED,
    PSECURITY_DESCRIPTOR, PUNICODE_STRING, PVOID, SIZE_T, TRUE, ULONG, UNICODE_STRING, USHORT,
    UTF8_STRING,
};

pub(crate) mod alloc;
pub(crate) mod macros;
pub(crate) mod timing;

pub(crate) type PsTerminateProcessType = unsafe extern "C" fn(*mut PEPROCESS, NTSTATUS) -> NTSTATUS;
pub(crate) type PsSetContextThreadType =
    unsafe extern "C" fn(*mut PETHREAD, PCONTEXT, KPROCESSOR_MODE) -> NTSTATUS;
pub(crate) static NT_PS_SET_CONTEXT_THREAD: AtomicPtr<PsSetContextThreadType> =
    AtomicPtr::new(null_mut());
pub(crate) static NT_PS_TERMINATE_PROCESS: AtomicPtr<PsTerminateProcessType> =
    AtomicPtr::new(null_mut());

#[allow(non_snake_case)]
pub(crate) unsafe fn PsSetContextThread(
    Thread: &PETHREAD,
    Context: PCONTEXT,
    ProcessorMode: KPROCESSOR_MODE,
) -> NTSTATUS {
    let mut status = 0;
    // the beautiful rust doesn't allow misaligned pointers.
    // it isn't my fault PsTerminateProcess is 0x8 unaligned. come on dude
    unsafe {
        asm!(
        "mov rcx, {0}",
        "mov rdx, {1}",
        "mov r8, {2}",
        "mov rax, {3}",
        "mov {4}, rax",
        in(reg) Thread,in(reg) Context, in(reg) ProcessorMode as u32, in(reg) NT_PS_TERMINATE_PROCESS.load(Ordering::Relaxed),
        inout(reg) status => status);
    }
    status as _
}

#[allow(non_snake_case)]
pub(crate) unsafe fn PsTerminateProcess(Process: &PEPROCESS, ExitCode: NTSTATUS) -> NTSTATUS {
    let mut status = 0;
    // the beautiful rust doesn't allow misaligned pointers.
    // it isn't my fault PsTerminateProcess is 0x8 unaligned. come on dude
    unsafe {
        asm!(
        "mov rcx, {0}",
        "mov rdx, {1}",
        "mov rax, {2}",
        "call rax",
        "mov {3}, rax",
        in(reg) Process,in(reg) ExitCode, in(reg) NT_PS_TERMINATE_PROCESS.load(Ordering::Relaxed),
        inout(reg) status => status);
    }
    status as _
}

#[link(name = "ntoskrnl")]
unsafe extern "C" {
    pub static PsLoadedModuleList: *mut _LDR_DATA_TABLE_ENTRY;

    #[allow(non_snake_case)]
    pub fn RtlCreateUserThread(
        ProcessHandle: HANDLE,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        CreateSuspended: BOOLEAN,
        ZeroBits: ULONG,
        MaximumStackSize: ULONG,
        CommittedStackSize: ULONG,
        StartAddress: PVOID,
        Parameter: PVOID,
        ThreadHandle: PHANDLE,
        ClientId: PCLIENT_ID,
    ) -> NTSTATUS;
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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

#[repr(C)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct _LDR_DATA_TABLE_ENTRY {
    pub InLoadOrderLinks: LIST_ENTRY,
    pub InMemoryOrderLinks: LIST_ENTRY,
    pub InInitializationOrderLinks: LIST_ENTRY,
    pub DllBase: PVOID,
    pub EntryPoint: PVOID,
    pub SizeOfImage: ULONG,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
    pub Flags: ULONG,
    pub LoadCount: USHORT,
    pub TlsIndex: USHORT,
    pub HashLinks: LIST_ENTRY,
    pub TimeDateStamp: ULONG,
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
