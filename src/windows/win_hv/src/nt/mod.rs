pub(crate) mod process;
pub(crate) mod worker;

use crate::win::{PsGetSetContextThreadInternal, PsLoadedModuleList, PsTerminateProcessType, NT_PS_GET_CONTEXT_THREAD_INTERNAL, NT_PS_SET_CONTEXT_THREAD_INTERNAL, NT_PS_TERMINATE_PROCESS, _LDR_DATA_TABLE_ENTRY};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use wdk_sys::ntddk::RtlGetVersion;
use wdk_sys::{PEPROCESS, RTL_OSVERSIONINFOW};

pub(crate) static NT_BUILD: AtomicU64 = AtomicU64::new(0);
pub(crate) static NT_BASE: AtomicPtr<u64> = AtomicPtr::new(null_mut());

///
/// # Get NT Info
///
/// The heart of HxPosed. Gets all the juicy stuff wdk doesn't give us.
///
/// ## Arguments
///
/// * `driver_section` - `DRIVER_OBJECT.DriverSection`
///
/// ## Return
///
/// - No values returned. [NT_BASE] and [NT_BUILD] are changed accordingly.
pub(crate) fn get_nt_info() {
    let mut info = RTL_OSVERSIONINFOW::default();
    let _ = unsafe { RtlGetVersion(&mut info) };

    NT_BUILD.store(info.dwBuildNumber as _, Ordering::Relaxed);

    unsafe {
        let entry = &mut *(PsLoadedModuleList);
        // first entry is always ntoskrnl
        let nt = &mut *(entry.InLoadOrderLinks.Flink as *mut _LDR_DATA_TABLE_ENTRY);
        NT_BASE.store(nt.DllBase as *mut u64, Ordering::Relaxed);

        NT_PS_TERMINATE_PROCESS.store(
            get_nt_proc::<PsTerminateProcessType>(NtProcedure::PsTerminateProcessProc),
            Ordering::Relaxed,
        );

        NT_PS_SET_CONTEXT_THREAD_INTERNAL.store(
            get_nt_proc::<PsGetSetContextThreadInternal>(NtProcedure::PspSetContextThreadInternal),
            Ordering::Relaxed,
        );

        NT_PS_GET_CONTEXT_THREAD_INTERNAL.store(
            get_nt_proc::<PsGetSetContextThreadInternal>(NtProcedure::PspGetContextThreadInternal),
            Ordering::Relaxed,
        );
    }
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
/// - This function panics if the NT version is not supported.
///
/// ## Return
/// - An absolute pointer to [`T`], if found.
///
pub(crate) unsafe fn get_nt_proc<T>(proc: NtProcedure) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    let base = NT_BASE.load(Ordering::Relaxed) as *mut u8;
    unsafe {
        base.add(match build {
        26200 /* 25H2 */ => {
            match proc {
                NtProcedure::PsTerminateProcessProc => 0x91f3d4,
                NtProcedure::PspGetContextThreadInternal => 0x00909940,
                NtProcedure::PspSetContextThreadInternal => 0x009095f0,
            }
        }
        _ => panic!("Unknown NT build {}", build)
    }) as *mut T
    }
}

///
/// # Get EPROCESS Field
///
/// Gets pointer to field of EPROCESS depending on NT version.
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
                    EProcessField::SignatureLevel => 0x5f8,
                    EProcessField::SectionSignatureLevel => 0x5f9,
                    EProcessField::MitigationFlags1 => 0x750,
                    EProcessField::MitigationFlags2 => 0x754,
                    EProcessField::MitigationFlags3 => 0x7d8
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
}

/// TODO: Document what those return
pub enum EProcessField {
    CreateTime,
    Token,
    SectionObject,
    SectionBaseAddress,
    Peb,
    SeAuditProcessCreationInfo,
    VadRoot,
    ExitTime,
    Protection,
    SignatureLevel,
    SectionSignatureLevel,
    MitigationFlags1,
    MitigationFlags2,
    MitigationFlags3,
}
