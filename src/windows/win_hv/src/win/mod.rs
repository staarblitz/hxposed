use ::alloc::boxed::Box;
use ::alloc::vec::Vec;
use core::arch::{asm, naked_asm};
use core::ffi::c_void;
use core::ptr::null_mut;
use core::str::FromStr;
use core::sync::atomic::{AtomicPtr, Ordering};
use wdk_sys::ntddk::{
    ExAllocatePool2, RtlCompareMemory, RtlCopyUnicodeString, RtlInitUTF8String,
    RtlUTF8StringToUnicodeString,
};
use wdk_sys::{
    BOOLEAN, CHAR, HANDLE, KPROCESSOR_MODE, LIST_ENTRY, NTSTATUS, OBJECT_ATTRIBUTES, PCLIENT_ID,
    PCONTEXT, PEPROCESS, PETHREAD, PHANDLE, POBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED, PSECURITY_DESCRIPTOR,
    PSIZE_T, PULONG, PUNICODE_STRING, PVOID, SIZE_T, STATUS_SUCCESS, TRUE, ULONG,
    UNICODE_STRING, USHORT, UTF8_STRING, _KPROCESS,
};

pub(crate) mod alloc;
pub(crate) mod danger;
pub(crate) mod macros;
pub(crate) mod timing;

pub(crate) type PsTerminateProcessType = unsafe extern "C" fn(PEPROCESS, NTSTATUS) -> NTSTATUS;
pub(crate) type PsTerminateThreadType = unsafe extern "C" fn(PETHREAD, NTSTATUS, CHAR) -> NTSTATUS;
pub(crate) type PsGetSetContextThreadInternal = unsafe extern "C" fn(
    PETHREAD,
    PCONTEXT,
    KPROCESSOR_MODE,
    KPROCESSOR_MODE,
    KPROCESSOR_MODE,
) -> NTSTATUS;
pub(crate) static NT_PS_TERMINATE_PROCESS: AtomicPtr<PsTerminateProcessType> =
    AtomicPtr::new(null_mut());
pub(crate) static NT_PS_GET_CONTEXT_THREAD_INTERNAL: AtomicPtr<PsGetSetContextThreadInternal> =
    AtomicPtr::new(null_mut());
pub(crate) static NT_PS_SET_CONTEXT_THREAD_INTERNAL: AtomicPtr<PsGetSetContextThreadInternal> =
    AtomicPtr::new(null_mut());
pub(crate) static NT_PS_TERMINATE_THREAD: AtomicPtr<PsTerminateThreadType> =
    AtomicPtr::new(null_mut());

#[allow(non_snake_case, unused)]
pub(crate) unsafe extern "C" fn PsTerminateProcess(
    Process: PEPROCESS,
    ExitCode: NTSTATUS,
) -> NTSTATUS {
    asm!("call _PsTerminateProcess",
        "ret", in("rax") NT_PS_TERMINATE_PROCESS.load(Ordering::Relaxed), options(nomem, nostack, preserves_flags));

    STATUS_SUCCESS // dummy
}

#[allow(non_snake_case, unused)]
pub(crate) unsafe extern "C" fn PspTerminateThread(
    Thread: PETHREAD,
    ExitCode: NTSTATUS,
    SomethingElse: CHAR,
) -> NTSTATUS {
    asm!("call _PspTerminateThread",
    "ret", in("rax") NT_PS_TERMINATE_PROCESS.load(Ordering::Relaxed), options(nomem, nostack, preserves_flags));

    STATUS_SUCCESS // dummy
}

#[allow(non_snake_case, unused)]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _PsTerminateProcess(Process: PEPROCESS, ExitCode: NTSTATUS) -> NTSTATUS {
    naked_asm!("jmp rax")
}

#[allow(non_snake_case, unused)]
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _PspTerminateThread(
    Thread: PETHREAD,
    ExitCode: NTSTATUS,
    SomethingElse: CHAR,
) -> NTSTATUS {
    naked_asm!("jmp rax")
}

#[allow(non_snake_case, unused)]
#[unsafe(naked)]
unsafe extern "C" fn _PspGetSetContextThreadInternal(
    Thread: PETHREAD,
    Context: PCONTEXT,
    Mode1: KPROCESSOR_MODE,
    Mode2: KPROCESSOR_MODE,
    Mode3: KPROCESSOR_MODE,
) -> NTSTATUS {
    naked_asm!("jmp rax")
}

#[allow(non_snake_case, unused)]
#[unsafe(naked)]
unsafe extern "C" fn _PspSetSetContextThreadInternal(
    Thread: PETHREAD,
    Context: PCONTEXT,
    Mode1: KPROCESSOR_MODE,
    Mode2: KPROCESSOR_MODE,
    Mode3: KPROCESSOR_MODE,
) -> NTSTATUS {
    naked_asm!("jmp rax")
}

pub(crate) const NT_CURRENT_PROCESS: HANDLE = -1 as _;

#[unsafe(naked)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn KeGetCurrentThread() -> PETHREAD {
    naked_asm!("mov rax, gs:[0x188]", "ret")
}

#[link(name = "ntoskrnl")]
unsafe extern "C" {
    pub static PsLoadedModuleList: *mut _LDR_DATA_TABLE_ENTRY;

    #[allow(non_snake_case)]
    pub fn MmCopyVirtualMemory(
        SourceProcess: PEPROCESS,
        SourceAddress: PVOID,
        TargetProcess: PEPROCESS,
        TargetAddress: PVOID,
        BufferSize: SIZE_T,
        PreviousMode: KPROCESSOR_MODE,
        ReturnSize: PSIZE_T,
    ) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn ZwSuspendThread(ThreadHandle: HANDLE, PreviousSuspendCount: PULONG) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn ZwProtectVirtualMemory(
        ProcessHandle: HANDLE,
        BaseAddress: *mut PVOID,
        RegionSize: PSIZE_T,
        NewProtection: ULONG,
        OldProtection: PULONG,
    ) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn ZwResumeThread(Thread: HANDLE, PreviousSuspendCount: PULONG) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn PsSetContextThread(
        Thread: PETHREAD,
        Context: PCONTEXT,
        AccessMode: KPROCESSOR_MODE,
    ) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn PsGetContextThread(
        Thread: PETHREAD,
        Context: PCONTEXT,
        AccessMode: KPROCESSOR_MODE,
    ) -> NTSTATUS;

    #[allow(non_snake_case)]
    pub fn RtlCreateUserThread(
        ProcessHandle: HANDLE,
        ThreadSecurityDescriptor: PSECURITY_DESCRIPTOR,
        CreateSuspended: BOOLEAN,
        ZeroBits: ULONG,
        MaximumStackSize: PULONG,
        CommittedStackSize: PULONG,
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
