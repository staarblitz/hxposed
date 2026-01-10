use crate::nt::lock::pushlock::PushLock;
use crate::nt::{EProcessField, EThreadField, get_eprocess_field, get_ethread_field};
use crate::utils::danger::DangerPtr;
use crate::win::PsTerminateProcess;
use alloc::vec::Vec;
use hxposed_core::services::types::process_fields::{
    MitigationOptions, ProcessProtection, ProcessSignatureLevel,
};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{IoGetCurrentProcess, ObOpenObjectByPointer, ObfDereferenceObject, ObfReferenceObject, PsGetProcessId, PsGetThreadId, PsLookupProcessByProcessId};
use wdk_sys::{
    _KTHREAD, HANDLE, LIST_ENTRY, NTSTATUS, PACCESS_TOKEN, PEPROCESS, PETHREAD, PLIST_ENTRY,
    PROCESS_ALL_ACCESS, PUNICODE_STRING, PsProcessType, STATUS_SUCCESS, UNICODE_STRING,
};
use crate::nt::context::ApcProcessContext;
use crate::utils::handlebox::HandleBox;

///
/// # Kernel Process
///
/// Abstraction over [`PEPROCESS`] to make the life easier.
#[allow(dead_code)]
pub struct NtProcess {
    pub nt_process: PEPROCESS,
    pub nt_path: PUNICODE_STRING,
    pub lock: PushLock,
    pub thread_list_head: PLIST_ENTRY,
    pub id: u32,
    pub uid: u64,
    pub owns: bool,
}

impl Drop for NtProcess {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                ObfDereferenceObject(self.nt_process as _);
            }
        }
    }
}

impl NtProcess {
    pub fn from_id(id: u32) -> Option<NtProcess> {
        let mut process = PEPROCESS::default();
        let status = unsafe { PsLookupProcessByProcessId(id as _, &mut process) };

        if status != STATUS_SUCCESS {
            return None;
        }

        Some(Self::open_process(process, true))
    }

    pub fn current() -> NtProcess {
        Self::open_process(unsafe { IoGetCurrentProcess() }, true)
    }

    pub fn from_ptr(process: PEPROCESS) -> Self {
        Self::open_process(process, false)
    }

    fn open_process(ptr: PEPROCESS, owns: bool) -> Self {
        let nt_path = unsafe {
            get_eprocess_field::<*mut UNICODE_STRING>(
                EProcessField::SeAuditProcessCreationInfo,
                ptr,
            )
        };
        Self {
            nt_process: ptr,
            nt_path: unsafe { *nt_path },
            id: unsafe { PsGetProcessId(ptr) } as _,
            lock: unsafe {
                PushLock::from_ptr(get_eprocess_field::<u64>(EProcessField::Lock, ptr))
            },
            thread_list_head: unsafe {
                get_eprocess_field::<LIST_ENTRY>(EProcessField::ThreadListHead, ptr)
            },
            uid: ptr as _,
            owns,
        }
    }

    pub fn open_handle(&self) -> Result<HandleBox, NTSTATUS> {
        let mut handle = HANDLE::default();
        match unsafe {
            ObOpenObjectByPointer(
                self.nt_process as _,
                0,
                Default::default(),
                PROCESS_ALL_ACCESS,
                *PsProcessType,
                KernelMode as _,
                &mut handle,
            )
        } {
            STATUS_SUCCESS => Ok(HandleBox::new(handle)),
            err => Err(err),
        }
    }

    pub fn begin_context(&self) -> ApcProcessContext {
        ApcProcessContext::begin(self.nt_process as _)
    }

    pub fn get_protection(&self) -> ProcessProtection {
        unsafe {
            *get_eprocess_field::<ProcessProtection>(EProcessField::Protection, self.nt_process)
        }
    }

    pub fn get_signers(&self) -> ProcessSignatureLevel {
        unsafe {
            *get_eprocess_field::<ProcessSignatureLevel>(
                EProcessField::SignatureLevels,
                self.nt_process,
            )
        }
    }

    pub fn get_mitigations(&self) -> MitigationOptions {
        unsafe {
            *get_eprocess_field::<MitigationOptions>(
                EProcessField::MitigationFlags1,
                self.nt_process,
            )
        }
    }

    pub fn get_token(&self) -> PACCESS_TOKEN {
        unsafe { *get_eprocess_field::<u64>(EProcessField::Token, self.nt_process) as _ }
    }

    pub fn set_protection(&mut self, protection: ProcessProtection) {
        let ptr = unsafe {
            get_eprocess_field::<ProcessProtection>(EProcessField::Protection, self.nt_process)
        };

        unsafe { ptr.write(protection) }
    }

    pub fn set_signers(&mut self, signers: ProcessSignatureLevel) {
        let ptr = unsafe {
            get_eprocess_field::<ProcessSignatureLevel>(
                EProcessField::SignatureLevels,
                self.nt_process,
            )
        };

        unsafe { ptr.write(signers) }
    }

    pub fn set_mitigations(&mut self, mitigations: MitigationOptions) {
        let ptr = unsafe {
            get_eprocess_field::<MitigationOptions>(
                EProcessField::MitigationFlags1,
                self.nt_process,
            )
        };

        unsafe { ptr.write(mitigations) }
    }

    pub fn set_token(&mut self, token: PACCESS_TOKEN) {
        let ptr = unsafe { get_eprocess_field::<u64>(EProcessField::Token, self.nt_process) };

        unsafe {
            ObfDereferenceObject(*ptr as _);
            ObfReferenceObject(token);
        }

        unsafe { ptr.write(token as _) }
    }

    pub fn get_threads(&self) -> Vec<u32> {
        self.lock.acquire_shared();

        let threads = DangerPtr {
            ptr: self.thread_list_head,
        };

        let first_entry = DangerPtr::<LIST_ENTRY> { ptr: threads.Blink };
        let mut current_entry = DangerPtr::<LIST_ENTRY> { ptr: threads.ptr };

        let mut thread_numbers = Vec::<u32>::new();

        while current_entry != first_entry {
            current_entry = DangerPtr::<LIST_ENTRY> {
                ptr: current_entry.Flink,
            };

            // now it gets tricky. let me explain.

            // the ThreadListHead field of _EPROCESS holds a LIST_ENTRY. what makes it deserve its own comments in this source is
            // that the list header for items are not the first field of the item.
            // for example, in 25h2, _ETHREAD's ThreadListEntry structure resides on offset 0x578.
            // so we have to go back exactly that many bytes to get the head of _ETHREAD object.

            // gets the real ETHREAD
            let thread = unsafe {
                get_ethread_field::<_KTHREAD>(
                    EThreadField::OffsetFromListEntry,
                    current_entry.ptr as _,
                ) as PETHREAD
            };

            thread_numbers.push(unsafe { PsGetThreadId(thread) as _ });
        }

        thread_numbers
    }

    pub fn kill(self, exit_code: NTSTATUS) -> Result<(), NTSTATUS> {
        match unsafe { PsTerminateProcess(self.nt_process, exit_code) } {
            STATUS_SUCCESS => Ok(()),
            err => Err(err),
        }
    }
}
