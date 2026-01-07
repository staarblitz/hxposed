use crate::nt::{get_eprocess_field, EProcessField};
use core::sync::atomic::AtomicPtr;
use wdk_sys::ntddk::{
    IoGetCurrentProcess, PsGetProcessId, PsLookupProcessByProcessId,
};
use wdk_sys::{PEPROCESS, STATUS_SUCCESS, UNICODE_STRING, _KPROCESS};

///
/// # Kernel Process
///
/// Abstraction over [`PEPROCESS`] to make the life easier.
#[allow(dead_code)]
pub struct NtProcess {
    pub nt_process: AtomicPtr<_KPROCESS>,
    pub nt_path: AtomicPtr<UNICODE_STRING>,
    pub id: u32,
}

impl NtProcess {
    #[allow(dead_code)]
    pub fn from_id(id: u32) -> Option<NtProcess> {
        let mut process = PEPROCESS::default();
        let status = unsafe { PsLookupProcessByProcessId(id as _, &mut process) };

        if status != STATUS_SUCCESS {
            return None;
        }

        Some(Self::open_process(process))
    }

    pub fn from_ptr(ptr: PEPROCESS) -> NtProcess {
        Self::open_process(ptr)
    }

    pub fn current() -> NtProcess {
        Self::open_process(unsafe { IoGetCurrentProcess() })
    }

    fn open_process(ptr: PEPROCESS) -> Self {
        let nt_path = unsafe {
            get_eprocess_field::<*mut UNICODE_STRING>(
                EProcessField::SeAuditProcessCreationInfo,
                ptr,
            )
        };
        Self {
            nt_process: AtomicPtr::new(ptr),
            nt_path: AtomicPtr::new(unsafe{*nt_path}),
            id: unsafe { PsGetProcessId(ptr) } as _,
        }
    }
}
