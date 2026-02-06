#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]
#![allow(unsafe_op_in_unsafe_fn)]

pub(crate) mod rtl_utils;
pub(crate) mod unicode_string;
pub(crate) mod winalloc;

use crate::utils::danger::DangerPtr;
use ::alloc::boxed::Box;
use ::alloc::vec::Vec;
use bitfield_struct::bitfield;
use core::arch::{asm, naked_asm};
use core::ffi::{c_char, c_void};
use core::fmt::{Formatter, LowerHex};
use core::mem;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};
use hxposed_core::services::types::security_fields::TokenPrivilege;

pub type PEPROCESS = *mut c_void;
pub type PKEVENT = *mut c_void;
pub type PACCESS_TOKEN = *mut c_void;
pub type PETHREAD = *mut c_void;
pub type HANDLE = *mut c_void;
pub type PVOID = *mut c_void;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ProcessorMode {
    UserMode = 1,
    KernelMode = 0,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StandardRights {
    Read = 0x00020000,
    Synchronize = 0x00100000,
    All = 0x001F0000,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyInformationClass {
    KeyBasicInformation,
    KeyNodeInformation,
    KeyFullInformation,
    KeyNameInformation,
    KeyCachedInformation,
    KeyFlagsInformation,
    KeyVirtualizationInformation,
    KeyHandleTagsInformation,
    KeyTrustInformation,
    KeyLayerInformation,
    MaxKeyInfoClass,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyValueInformationClass {
    KeyValueBasicInformation,
    KeyValueFullInformation,
    KeyValuePartialInformation,
    KeyValueFullInformationAlign64,
    KeyValuePartialInformationAlign64,
    KeyValueLayerInformation,
    MaxKeyValueInfoClass,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum KeyAccessRights {
    QueryValue = 0x0001,
    SetValue = 0x0002,
    CreateSubKey = 0x0004,
    EnumerateSubKeys = 0x0008,
    Notify = 0x0010,
    CreateLink = 0x0020,
    Wow6432Key = 0x0200,
    Wow6464Key = 0x0100,
    Wow64Res = 0x300,

    Read = (StandardRights::Read as u32
        | KeyAccessRights::QueryValue as u32
        | KeyAccessRights::EnumerateSubKeys as u32
        | KeyAccessRights::Notify as u32)
        & (!(StandardRights::Synchronize as u32)),

    Write = (StandardRights::Read as u32
        | KeyAccessRights::SetValue as u32
        | KeyAccessRights::CreateSubKey as u32)
        & (!(StandardRights::Synchronize as u32)),

    All = (StandardRights::All as u32
        | KeyAccessRights::QueryValue as u32
        | KeyAccessRights::Notify as u32
        | KeyAccessRights::CreateLink as u32
        | KeyAccessRights::EnumerateSubKeys as u32
        | KeyAccessRights::SetValue as u32)
        & (!(StandardRights::Synchronize as u32)),
}

#[repr(u8)]
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Boolean {
    True = 1,
    #[default]
    False = 0,
}

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        match value {
            true => Boolean::True,
            false => Boolean::False,
        }
    }
}

#[repr(u64)]
pub enum PoolFlags {
    NonPaged = 0x0000000000000040,
}

#[repr(u32)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum NtStatus {
    Success = 0,
    TooLate = 0xC0000189,
    Alerted = 0x00000101,
    TimeOut = 0x00000102,
    UserApc = 0x000000C0,
    Unsuccessful = 0xC0000001,
    NotAllocated = 0xC00000A0,
    AccessViolation = 0xC0000005,
    BufferTooSmall = 0xc0000023
}

impl LowerHex for NtStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(stringify!(self.0))
    }
}

#[repr(u32)]
pub enum EventType {
    NotificationEvent,
    SynchronizationEvent,
}

#[repr(u32)]
pub enum WaitReason {
    Executive,
}

#[repr(u32)]
pub enum SecurityImpersonationLevel {
    SecurityAnonymous,
    SecurityIdentification,
    SecurityImpersonation,
    SecurityDelegation,
}

#[repr(u32)]
pub enum PagePriority {
    LowPagePriority,
    NormalPagePriority = 16,
    HighPagePriority = 32,
    // These are included because they can be used.
    NoWrite = 0x80000000,
    NoExecute = 0x40000000,
    WithGuardPtes = 0x20000000,
}

#[repr(u32)]
pub enum MemoryCacheType {
    MmNonCached,
    MmCached,
    MmWriteCombined,
    MmHardwareCoherentCached,
    MmNonCachedUnordered,
    MmUSWCCached,
    MmMaximumCacheType,
    MmNotMapped,
}

#[repr(u32)]
#[derive(Clone, Default, Debug)]
pub enum MdlFlags {
    #[default]
    None = 0,
    MappedToSystemVa = 0x0001,
    PagesLocked = 0x0002,
    SourceIsNonpagedPool = 0x0004,
    AllocatedFixedSize = 0x0008,
    Partial = 0x0010,
    PartialHasBeenMapped = 0x0020,
    IoPageRead = 0x0040,
    WriteOperation = 0x0080,
    LockedPageTables = 0x0100,
    FreeExtraPtes = 0x0200,
    DescribesAwe = 0x0400,
    IoSpace = 0x0800,
    NetworkHeader = 0x1000,
    MappingCanFail = 0x2000,
    AllocatedMustSucceed = 0x4000,
    Internal = 0x8000,
}

pub(crate) type PsTerminateProcessType = unsafe extern "C" fn(PEPROCESS, NtStatus) -> NtStatus;
pub(crate) type PsTerminateThreadType = unsafe extern "C" fn(PETHREAD, NtStatus, i8) -> NtStatus;
pub(crate) type ExpLookupHandleTableEntryType =
    unsafe extern "C" fn(PHANDLE_TABLE, _EXHANDLE) -> *mut u64;
pub(crate) type ExCreateHandleType = unsafe extern "C" fn(PHANDLE_TABLE, PVOID) -> *mut u64;

#[unsafe(no_mangle)]
pub(crate) static mut NT_PS_TERMINATE_PROCESS: u64 = 0;
#[unsafe(no_mangle)]
pub(crate) static mut NT_PS_GET_CONTEXT_THREAD_INTERNAL: u64 = 0;
#[unsafe(no_mangle)]
pub(crate) static mut NT_PS_SET_CONTEXT_THREAD_INTERNAL: u64 = 0;
#[unsafe(no_mangle)]
pub(crate) static mut NT_PS_TERMINATE_THREAD: u64 = 0;
#[unsafe(no_mangle)]
pub(crate) static mut NT_EXP_LOOKUP_HANDLE_TABLE_ENTRY: u64 = 0;
#[unsafe(no_mangle)]
pub(crate) static mut NT_EX_CREATE_HANDLE: u64 = 0;

pub unsafe extern "C" fn ExpLookupHandleTableEntry(
    Table: PHANDLE_TABLE,
    Handle: _EXHANDLE,
) -> *mut u64 {
    let func: ExpLookupHandleTableEntryType = mem::transmute(NT_EXP_LOOKUP_HANDLE_TABLE_ENTRY);
    func(Table, Handle)
}

pub unsafe extern "C" fn ExCreateHandle(Table: PHANDLE_TABLE, ObjectHeader: PVOID) -> *mut u64 {
    let func: ExCreateHandleType = mem::transmute(NT_EX_CREATE_HANDLE);
    func(Table, ObjectHeader)
}

pub unsafe extern "C" fn PsTerminateProcess(Process: PEPROCESS, ExitCode: NtStatus) -> NtStatus {
    let func: PsTerminateProcessType = mem::transmute(NT_PS_TERMINATE_PROCESS);
    func(Process, ExitCode)
}

pub unsafe extern "C" fn PspTerminateThread(
    Thread: PETHREAD,
    ExitCode: NtStatus,
    SomethingElse: i8,
) -> NtStatus {
    let func: PsTerminateThreadType = mem::transmute(NT_PS_TERMINATE_THREAD);
    func(Thread, ExitCode, SomethingElse)
}

pub(crate) const NT_CURRENT_PROCESS: HANDLE = -1 as _;

#[unsafe(naked)]
pub unsafe extern "C" fn KeGetCurrentThread() -> PETHREAD {
    naked_asm!("mov rax, gs:[0x188]", "ret")
}

#[link(name = "ntoskrnl")]
unsafe extern "C" {
    pub static PsLoadedModuleList: *mut LDR_DATA_TABLE_ENTRY;

    pub fn DbgPrint(Str: *const c_char, ...);

    pub fn PsLookupProcessByProcessId(Id: HANDLE, Process: *mut PEPROCESS) -> NtStatus;
    pub fn PsLookupThreadByThreadId(Id: HANDLE, Process: *mut PETHREAD) -> NtStatus;
    pub fn PsReferencePrimaryToken(Process: PEPROCESS) -> PACCESS_TOKEN;
    pub fn PsReferenceImpersonationToken(
        Thread: PETHREAD,
        CopyOnOpen: *mut Boolean,
        EffectiveOnly: *mut Boolean,
        Level: *mut SecurityImpersonationLevel,
    ) -> PACCESS_TOKEN;
    pub fn PsSetCreateProcessNotifyRoutineEx(Routine: PVOID, Remove: Boolean) -> NtStatus;
    pub fn PsGetProcessId(Process: PEPROCESS) -> HANDLE;
    pub fn PsGetThreadId(Thread: PETHREAD) -> HANDLE;

    pub fn KeDelayExecutionThread(WaitMode: ProcessorMode, Alertable: Boolean, interval: *mut i64);

    pub fn ExAllocatePool2(Flags: PoolFlags, Bytes: usize, Tag: u32) -> PVOID;
    pub fn ExFreePool(Pool: PVOID);
    pub fn ExReleasePushLockExclusiveEx(Lock: *mut u64, Flags: u32);
    pub fn ExReleasePushLockSharedEx(Lock: *mut u64, Flags: u32);
    pub fn ExAcquirePushLockExclusiveEx(Lock: *mut u64, Flags: u32);
    pub fn ExAcquirePushLockSharedEx(Lock: *mut u64, Flags: u32);

    pub fn IoGetCurrentProcess() -> PEPROCESS;
    pub fn IoFreeMdl(Mdl: *mut MDL);
    pub fn IoAllocateMdl(
        Va: PVOID,
        Length: u32,
        SecondBuffer: Boolean,
        ChargeQuota: Boolean,
        Irp: PVOID,
    ) -> *mut MDL;

    pub fn MmFreePagesFromMdl(Mdl: *mut MDL);
    pub fn MmUnmapLockedPages(Va: PVOID, Mdl: *mut MDL);
    pub fn MmBuildMdlForNonPagedPool(Mdl: *mut MDL);
    pub fn MmAllocatePagesForMdlEx(
        Low: i64,
        High: i64,
        Skip: i64,
        Total: usize,
        CacheType: MemoryCacheType,
        Flags: u32,
    ) -> *mut MDL;
    pub fn MmProtectMdlSystemAddress(Mdl: *mut MDL, Protection: u32) -> NtStatus;
    pub fn MmMapLockedPagesSpecifyCache(
        Mdl: *mut MDL,
        AccessMode: ProcessorMode,
        CacheType: MemoryCacheType,
        RequestedAddress: PVOID,
        BugCheck: Boolean,
        Priority: u32,
    ) -> PVOID;
    pub fn MmIsAddressValid(Va: PVOID) -> Boolean;
    pub fn MmAllocateContiguousMemory(Size: usize, HighestAcceptable: u64) -> PVOID;
    pub fn MmFreeContiguousMemory(Va: PVOID);
    pub fn MmGetPhysicalAddress(Va: PVOID) -> u64;
    pub fn MmGetVirtualForPhysical(Pa: u64) -> PVOID;

    pub fn KeBugCheckEx(Code: u64, Param1: u64, Param2: u64, Param3: u64, Param4: u64) -> !;
    pub fn KeStackAttachProcess(Process: PEPROCESS, ApcState: *mut KAPC_STATE);
    pub fn KeUnstackDetachProcess(ApcState: *mut KAPC_STATE);
    pub fn KeInitializeEvent(Event: PKEVENT, Type: EventType, State: Boolean);
    pub fn KeSetEvent(Event: PKEVENT, Priority: u32, Wait: Boolean) -> u32;
    pub fn KeWaitForSingleObject(
        Object: PVOID,
        Reason: WaitReason,
        Mode: ProcessorMode,
        Alertable: Boolean,
        Timeout: *mut i64,
    ) -> NtStatus;
    pub fn KeQueryActiveProcessorCountEx(GroupNumber: u16) -> u32;
    pub fn KeGetProcessorNumberFromIndex(ProcIndex: u32, ProcNumber: *mut PROCESSOR_NUMBER) -> NtStatus;
    pub fn KeSetSystemGroupAffinityThread(Affinity: *mut GROUP_AFFINITY, PreviousAffinity: *mut GROUP_AFFINITY);
    pub fn KeRevertToUserGroupAffinityThread(Previous: *mut GROUP_AFFINITY);

    pub fn ZwOpenKey(
        Handle: *mut HANDLE,
        AccessMask: KeyAccessRights,
        Attributes: *mut OBJECT_ATTRIBUTES,
    ) -> NtStatus;

    pub fn ZwQueryKey(
        Handle: HANDLE,
        InfoClass: KeyInformationClass,
        Information: PVOID,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NtStatus;

    pub fn ZwQueryValueKey(
        Handle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        InfoClass: KeyValueInformationClass,
        Information: PVOID,
        Length: u32,
        ReturnLength: *mut u32,
    ) -> NtStatus;

    pub fn ZwClose(Handle: HANDLE) -> NtStatus;
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct GROUP_AFFINITY {
    pub Mask: u64,
    pub Group: u16,
    pub Reserved: [u16;3]
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8
}

#[repr(C)]
#[derive(Default, Clone, Debug)]
pub struct MDL {
    pub Next: *mut MDL,
    pub Size: u16,
    pub MdlFlags: MdlFlags,
    pub Process: PEPROCESS,
    pub MappedSystemVa: PVOID,
    pub StartVa: PVOID,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct KAPC_STATE {
    pub ApcListHead: [LIST_ENTRY; 2],
    pub Process: PEPROCESS,
    pub KernelApcInProgress: Boolean,
    pub KernelApcPending: Boolean,
    pub UserApcPending: Boolean,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct KEY_FULL_INFORMATION {
    pub LastWriteTime: u64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub SubKeys: u32,
    pub MaxNameLen: u32,
    pub MaxClassLen: u32,
    pub Values: u32,
    pub MaxValueNameLen: u32,
    pub MaxValueDataLen: u32,
    pub Class: [u16; 1],
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct KEY_VALUE_FULL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct UNICODE_STRING {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Buffer: *mut u16,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub InLoadOrderLinks: *mut LIST_ENTRY,
    pub InMemoryOrderLinks: *mut LIST_ENTRY,
    pub InInitOrderLinks: *mut LIST_ENTRY,
    pub DllBase: PVOID,
    pub EntryPoint: PVOID,
    pub SizeOfImage: usize,
    pub FullDllName: UNICODE_STRING,
    pub BaseDllName: UNICODE_STRING,
    // rest is unneeded.
}

#[repr(u32)]
#[derive(Copy, Clone, Default)]
pub enum ObjectAttributes {
    #[default]
    None = 0,
    Inherit = 0x00000002,
    KernelHandle = 0x00000200,
}
#[repr(C)]
#[derive(Default, Clone)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: HANDLE,
    pub ObjectName: *mut UNICODE_STRING,
    pub Attributes: ObjectAttributes,
    pub SecurityDescriptor: PVOID,
    pub SecurityQOS: PVOID,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct LIST_ENTRY {
    pub Flink: *mut LIST_ENTRY,
    pub Blink: *mut LIST_ENTRY,
}

/// This structure hasn't changed for 3 major updates.
/// I think it's safe to manually use it.
#[repr(C)]
#[derive(Default, Clone)]
pub struct _HANDLE_TABLE {
    pub NextHandleNeedingPool: u32,
    pub ExtraInfoPages: i32,
    pub TableCode: u64,
    pub QuotaProcess: PEPROCESS,
    pub HandleTableList: LIST_ENTRY,
    pub UniqueProcessId: u32,
    pub Flags: u32,
    pub HandleContentionEvent: u64, // push lock
    pub HandleTableLock: u64,       // push lock
                                    // rest is not required
}

pub type PHANDLE_TABLE = *mut _HANDLE_TABLE;

#[repr(C)]
#[derive(Default, Clone)]
pub struct _EXHANDLE {
    pub Value: u64,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct _SEP_TOKEN_PRIVILEGES {
    pub Present: TokenPrivilege,
    pub Enabled: TokenPrivilege,
    pub EnabledByDefault: TokenPrivilege,
}
