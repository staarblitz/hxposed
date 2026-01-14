use crate::nt::lock::pushlock::PushLock;
use crate::nt::{EThreadField, get_ethread_field};
use crate::utils::handlebox::HandleBox;
use crate::win::{KeGetCurrentThread, PspTerminateThread};
use bit_field::BitField;
use core::hash::{Hash, Hasher};
use core::ptr::null_mut;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{
    ObOpenObjectByPointer, ObfDereferenceObject, ObfReferenceObject, PsCreateSystemThread,
    PsGetThreadId, PsLookupThreadByThreadId, PsReferenceImpersonationToken, ZwClose,
};
use wdk_sys::{
    FALSE, HANDLE, NTSTATUS, PACCESS_TOKEN, PETHREAD, PKSTART_ROUTINE, PVOID, PsThreadType,
    SECURITY_IMPERSONATION_LEVEL, STATUS_SUCCESS, THREAD_ALL_ACCESS,
};

pub struct NtThread {
    pub nt_thread: PETHREAD,
    pub id: u32,
    pub lock: PushLock,
    pub owns: bool,
}

impl Hash for NtThread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.nt_thread as _);
    }
}

unsafe impl Send for NtThread {}
unsafe impl Sync for NtThread {}

impl Drop for NtThread {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                ObfDereferenceObject(self.nt_thread as _);
            }
        }
    }
}

impl NtThread {
    pub fn from_id(id: u32) -> Option<NtThread> {
        let mut process = PETHREAD::default();
        let status = unsafe { PsLookupThreadByThreadId(id as _, &mut process) };

        if status != STATUS_SUCCESS {
            return None;
        }

        Some(Self::open_thread(process, true))
    }

    pub fn current() -> NtThread {
        Self::open_thread(unsafe { KeGetCurrentThread() }, true)
    }

    pub fn from_ptr(process: PETHREAD) -> Self {
        Self::open_thread(process, false)
    }

    fn open_thread(ptr: PETHREAD, owns: bool) -> Self {
        Self {
            nt_thread: ptr,
            id: unsafe { PsGetThreadId(ptr) } as _,
            lock: unsafe { PushLock::from_ptr(get_ethread_field::<u64>(EThreadField::Lock, ptr)) },
            owns,
        }
    }

    pub fn open_handle(&self) -> Result<HandleBox, NTSTATUS> {
        let mut handle = HANDLE::default();
        match unsafe {
            ObOpenObjectByPointer(
                self.nt_thread as _,
                0,
                Default::default(),
                THREAD_ALL_ACCESS,
                *PsThreadType,
                KernelMode as _,
                &mut handle,
            )
        } {
            STATUS_SUCCESS => Ok(HandleBox::new(handle)),
            err => Err(err),
        }
    }

    pub fn create(thread_entry: PKSTART_ROUTINE, start_context: Option<PVOID>) -> NTSTATUS {
        let mut handle = HANDLE::default();
        match unsafe {
            PsCreateSystemThread(
                &mut handle,
                THREAD_ALL_ACCESS,
                Default::default(),
                Default::default(),
                Default::default(),
                thread_entry,
                start_context.unwrap_or(null_mut()),
            )
        } {
            STATUS_SUCCESS => unsafe {
                let _ = ZwClose(handle);
                STATUS_SUCCESS
            },
            err => {
                log::error!("Error creating worker thread: {:?}", err);
                err
            }
        }
    }

    pub fn get_impersonation_info(&self) -> bool {
        unsafe { *get_ethread_field::<u32>(EThreadField::CrossThreadFlags, self.nt_thread) }
            .get_bit(3)
    }

    pub fn get_adjusted_client_token(&self) -> PACCESS_TOKEN {
        let mut copy_on_open = FALSE as _;
        let mut effective_only = FALSE as _;
        let mut impersonation_level = SECURITY_IMPERSONATION_LEVEL::default();

        unsafe {
            PsReferenceImpersonationToken(
                self.nt_thread as _,
                &mut copy_on_open,
                &mut effective_only,
                &mut impersonation_level,
            )
        }
    }

    pub fn set_adjusted_client_token(&mut self, token: PACCESS_TOKEN) {
        self.lock.acquire_exclusive();

        let current_token = unsafe {
            get_ethread_field::<PACCESS_TOKEN>(
                EThreadField::AdjustedClientToken,
                self.nt_thread as _,
            )
        };

        unsafe {
            ObfDereferenceObject(*current_token as _);
            //  its now being referenced by another process. we need to increase its reference count
            ObfReferenceObject(token);
        }

        unsafe {
            current_token.write(token);
        }
    }

    pub fn kill(self, exit_code: NTSTATUS) -> Result<(), NTSTATUS> {
        match unsafe { PspTerminateThread(self.nt_thread as _, exit_code, 1) } {
            STATUS_SUCCESS => Ok(()),
            err => Err(err),
        }
    }
}
