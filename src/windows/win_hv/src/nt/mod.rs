#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub(crate) mod callback;
pub(crate) mod context;
pub(crate) mod guard;
mod lock;
pub(crate) mod mdl;
pub(crate) mod probe;
pub(crate) mod process;
mod registry;
pub(crate) mod thread;
pub(crate) mod token;
pub(crate) mod worker;

use crate::win::*;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use hxposed_core::services::types::security_fields::TokenPrivilege;
use wdk_sys::ntddk::{PsGetVersion, PsLookupProcessByProcessId, PsReferencePrimaryToken};
use wdk_sys::{PACCESS_TOKEN, PEPROCESS, PETHREAD, PVOID, ULONG};

pub(crate) static NT_BUILD: AtomicU64 = AtomicU64::new(0);
pub(crate) static NT_BASE: AtomicPtr<u64> = AtomicPtr::new(null_mut());
pub(crate) static SYSTEM_TOKEN: AtomicPtr<u64> = AtomicPtr::new(null_mut());

pub(crate) type PSEP_LOGON_SESSION_REFERENCES = *mut _SEP_LOGON_SESSION_REFERENCES;

pub(crate) type _SEP_LOGON_SESSION_REFERENCES = u64;

///
/// # Get NT Info
///
/// The heart of HxPosed. Gets all the juicy stuff wdk doesn't give us.
///
/// ## Arguments
///
/// * `custom` - If [`Some`], used. If not, nt version is dynamically fetched.
///
/// ## Return
///
/// - No values returned. [`NT_BASE`] and [`NT_BUILD`] are changed accordingly.
#[allow(static_mut_refs)]
pub(crate) fn get_nt_info(custom: Option<u32>) -> Result<(), ()> {
    let build_number = {
        if let Some(build_number) = custom {
            build_number
        } else {
            let mut build = ULONG::default();
            unsafe { PsGetVersion(null_mut(), null_mut(), &mut build, null_mut()) };

            build
        }
    };

    NT_BUILD.store(build_number as _, Ordering::Relaxed);

    match build_number {
        26200 => {}
        _ => {
            log::error!(
                "HxPosed does not support your Windows version: {}",
                build_number
            );
            return Err(());
        }
    }

    get_system_token();

    unsafe {
        NT_BASE.store(get_nt_base() as _, Ordering::Relaxed);

        NT_PS_TERMINATE_PROCESS =
            get_nt_proc::<PsTerminateProcessType>(NtProcedure::PsTerminateProcessProc) as _;

        // NT_PS_SET_CONTEXT_THREAD_INTERNAL =
        //     get_nt_proc::<PsGetSetContextThreadInternal>(NtProcedure::PspSetContextThreadInternal)
        //         as _;
        //
        // NT_PS_GET_CONTEXT_THREAD_INTERNAL =
        //     get_nt_proc::<PsGetSetContextThreadInternal>(NtProcedure::PspGetContextThreadInternal)
        //         as _;

        NT_PS_TERMINATE_THREAD =
            get_nt_proc::<PsTerminateThreadType>(NtProcedure::PspTerminateThreadByPointer) as _;
    }

    Ok(())
}
pub(crate) fn get_nt_base() -> PVOID {
    unsafe {
        let entry = &mut *(PsLoadedModuleList);
        // first entry is always ntoskrnl
        let nt = &mut *(entry.InLoadOrderLinks.Flink as *mut _LDR_DATA_TABLE_ENTRY);
        nt.DllBase
    }
}

fn get_system_token() {
    let mut system = PEPROCESS::default();
    let _ = unsafe { PsLookupProcessByProcessId(4 as _, &mut system) };

    // for some reason, cannot link external symbol PsInitialSystemProcess. huh
    SYSTEM_TOKEN.store(
        unsafe { PsReferencePrimaryToken(system) } as _,
        Ordering::Relaxed,
    );
}

///
/// # Get NT Procedure
///
/// Gets the function at ntosrkrnl.
///
/// ## Arguments
/// * `proc` - Procedure to get pointer of. See [NtProcedure]
///
/// ## Panic
/// * This function panics if the NT version is not supported.
///
/// ## Return
/// * An absolute pointer to [`T`], if found.
///
pub(crate) unsafe fn get_nt_proc<T>(proc: NtProcedure) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    let base = NT_BASE.load(Ordering::Relaxed) as *mut u8;
    unsafe {
        base.add(match build {
            26200 /* 25H2 */ => {
                match proc {
                    NtProcedure::PsTerminateProcessProc => 0x91f3d4,
                    NtProcedure::PspGetContextThreadInternal => 0x909940,
                    NtProcedure::PspSetContextThreadInternal => 0x9095f0,
                    NtProcedure::PspTerminateThreadByPointer => 0x8f48f0,
                }
            }
            _ => panic!("Unknown NT build {}", build)
        }) as *mut T
    }
}

///
/// # Get `EPROCESS` Field
///
/// Gets pointer to field of `EPROCESS` depending on NT version.
///
/// ## Arguments
/// * `field` - Field you want to acquire pointer to. See [`EProcessField`]
/// * `process` - Process object to get pointer from.
///
/// ## Panic
/// - This function panics if the NT version is not supported.
///
/// ## Returns
/// - Absolute **pointer** to the field, in [`T`].
pub(crate) unsafe fn get_eprocess_field<T: 'static>(
    field: EProcessField,
    process: PEPROCESS,
) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    unsafe {
        process.byte_offset(match build {
            26200 /* 25H2 */ => {
                match field {
                    EProcessField::CreateTime => 0x1f8,
                    EProcessField::Token => 0x248,
                    EProcessField::SectionObject => 0x2f8,
                    EProcessField::SectionBaseAddress => 0x2b0,
                    EProcessField::Peb => 0x2e0,
                    EProcessField::SeAuditProcessCreationInfo => 0x350,
                    EProcessField::VadRoot => 0x558,
                    EProcessField::ExitTime => 0x5c0,
                    EProcessField::Protection => 0x5fa,
                    EProcessField::SignatureLevels => 0x5f8,
                    EProcessField::MitigationFlags1 => 0x750,
                    EProcessField::MitigationFlags2 => 0x754,
                    EProcessField::MitigationFlags3 => 0x7d8,
                    EProcessField::ThreadListHead => 0x370,
                    EProcessField::Lock => 0x1c8,
                }
            }
            _ => {
                panic!("Unknown NT build {}", build)
            }
        }) as *mut T
    }
}

///
/// # Get `ETHREAD` Field
///
/// Gets pointer to field of `ETHREAD` depending on NT version.
///
/// ## Arguments
/// * `field` - Field you want to acquire pointer to. See [`EThreadField`]
/// * `thread` - Thread object to get pointer from.
///
/// ## Panic
/// - This function panics if the NT version is not supported.
///
/// ## Returns
/// - Absolute **pointer** to the field, in [`T`].
pub(crate) unsafe fn get_ethread_field<T: 'static>(
    field: EThreadField,
    thread: PETHREAD,
) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    unsafe {
        thread.byte_offset(match build {
            26200 /* 25H2 */ => {
                match field {
                    EThreadField::Lock => 0x590,
                    EThreadField::OffsetFromListEntry => -0x578, // returns the pointer to actual ETHREAD
                    EThreadField::ClientId => 0x508,
                    EThreadField::CrossThreadFlags => 0x5a0,
                    EThreadField::AdjustedClientToken => 0x648
                }
            }
            _ => {
                panic!("Unknown NT build {}", build)
            }
        }) as *mut T
    }
}

///
/// # Get `ACCESS_TOKEN` Field
///
/// Gets pointer to field of `ACCESS_TOKEN` depending on NT version.
///
/// ## Arguments
/// * `field` - Field you want to acquire pointer to. See [`AccessTokenField`]
/// * `thread` - Thread object to get pointer from.
///
/// ## Panic
/// - This function panics if the NT version is not supported.
///
/// ## Returns
/// - Absolute **pointer** to the field, in [`T`].
pub(crate) unsafe fn get_access_token_field<T: 'static>(
    field: AccessTokenField,
    token: PACCESS_TOKEN,
) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    unsafe {
        token.byte_offset(match build {
            26200 /* 25H2 */ => {
                match field {
                    AccessTokenField::TokenSource => 0x0,
                    AccessTokenField::LogonSession => 0xd8,
                    AccessTokenField::Type => 0xc0,
                    AccessTokenField::IntegrityLevelIndex => 0xd0,
                    AccessTokenField::MandatoryPolicy => 0xd4,
                    AccessTokenField::ImpersonationLevel => 0xc4,
                    AccessTokenField::Privileges => 0x40,
                }
            }
            _ => {
                panic!("Unknown NT build {}", build)
            }
        }) as *mut T
    }
}

///
/// # Get `_SEP_LOGON_SESSION_REFERENCES` Field
///
/// Gets pointer to field of `_SEP_LOGON_SESSION_REFERENCES` depending on NT version.
///
/// ## Arguments
/// * `field` - Field you want to acquire pointer to. See [`LogonSessionField`]
/// * `thread` - Thread object to get pointer from.
///
/// ## Panic
/// - This function panics if the NT version is not supported.
///
/// ## Returns
/// - Absolute **pointer** to the field, in [`T`].
pub(crate) unsafe fn get_logon_session_field<T: 'static>(
    field: LogonSessionField,
    token: PSEP_LOGON_SESSION_REFERENCES,
) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    unsafe {
        token.byte_offset(match build {
            26200 /* 25H2 */ => {
                match field {
                    LogonSessionField::LogonId => 0x8,
                    LogonSessionField::Flags => 0x20,
                    LogonSessionField::Token => 0x30,
                    LogonSessionField::AccountName => 0x38,
                    LogonSessionField::AuthorityName => 0x48,
                }
            }
            _ => {
                panic!("Unknown NT build {}", build)
            }
        }) as *mut T
    }
}

pub enum NtProcedure {
    PsTerminateProcessProc,
    PspSetContextThreadInternal,
    PspGetContextThreadInternal,
    PspTerminateThreadByPointer,
}

pub enum LogonSessionField {
    LogonId,
    Flags,
    Token,
    AccountName,
    AuthorityName,
}

pub enum AccessTokenField {
    TokenSource,
    LogonSession,
    Type,
    IntegrityLevelIndex,
    MandatoryPolicy,
    ImpersonationLevel,
    Privileges,
}

pub enum EThreadField {
    Lock,
    OffsetFromListEntry,
    ClientId,
    CrossThreadFlags,
    AdjustedClientToken,
}

/// TODO: Document what those return
pub enum EProcessField {
    Lock,
    CreateTime,
    Token,
    SectionObject,
    SectionBaseAddress,
    Peb,
    SeAuditProcessCreationInfo,
    VadRoot,
    ExitTime,
    Protection,
    ThreadListHead,
    SignatureLevels,
    MitigationFlags1,
    MitigationFlags2,
    MitigationFlags3,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
#[allow(non_snake_case, non_camel_case_types)]
pub struct _SEP_TOKEN_PRIVILEGES {
    pub Present: TokenPrivilege,
    pub Enabled: TokenPrivilege,
    pub EnabledByDefault: TokenPrivilege,
}
