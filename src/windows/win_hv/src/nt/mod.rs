pub(crate) mod process;

use core::sync::atomic::{AtomicU64, Ordering};
use wdk_sys::ntddk::RtlGetVersion;
use wdk_sys::{PEPROCESS, RTL_OSVERSIONINFOW};

pub(crate) static NT_BUILD: AtomicU64 = AtomicU64::new(0);

pub(crate) fn get_nt_info() {
    let mut info = RTL_OSVERSIONINFOW::default();
    unsafe { RtlGetVersion(&mut info) };

    NT_BUILD.store(info.dwBuildNumber as _, Ordering::Relaxed);
}


///
/// # Get EPROCESS Field
///
/// Gets pointer to field of EPROCESS depending on NT version.
///
/// ## Arguments
/// field - Field you want to acquire pointer to. See [EProcessField]
///
/// process - Process object to get pointer from.
///
/// ## Returns
/// Absolute **pointer** to the field, in T.
pub(crate) unsafe fn get_eprocess_field<T: 'static>(field: EProcessField, process: PEPROCESS) -> *mut T {
    let build = NT_BUILD.load(Ordering::Relaxed);
    match build {
        26200 /* 25H2 */ => unsafe {
            process.byte_offset(match field {
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
            }) as *mut T
        }
        _ => {
            panic!("Unknown NT build {}", build)
        }
    }
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