use crate::nt::lock::pushlock::PushLock;
use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::nt::{EThreadField, get_ethread_field};
use crate::utils::handlebox::HandleBox;
use crate::win::{
    Boolean, HANDLE, KeGetCurrentThread, NtStatus, PACCESS_TOKEN, PETHREAD, PsGetThreadId,
    PsLookupThreadByThreadId, PsReferenceImpersonationToken, PspTerminateThread,
    SecurityImpersonationLevel,
};
use bit_field::BitField;
use core::hash::{Hash, Hasher};
use core::ptr::null_mut;

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
                NtObject::<u64>::decrement_ref_count(self.nt_thread as _);
            }
        }
    }
}

impl NtThread {
    pub fn from_id(id: u32) -> Option<NtThread> {
        let mut process = PETHREAD::default();
        let status = unsafe { PsLookupThreadByThreadId(id as _, &mut process) };

        if status != NtStatus::Success {
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

    pub fn open_handle(&self) -> HandleBox {
        HandleBox::new(
            NtObject::create_handle(self.nt_thread, NtProcess::current().get_handle_table())
                .unwrap(),
        )
    }

    pub fn get_impersonation_info(&self) -> bool {
        unsafe { *get_ethread_field::<u32>(EThreadField::CrossThreadFlags, self.nt_thread) }
            .get_bit(3)
    }

    pub fn get_adjusted_client_token(&self) -> PACCESS_TOKEN {
        let mut copy_on_open = Boolean::False;
        let mut effective_only = Boolean::False;
        let mut impersonation_level = SecurityImpersonationLevel::SecurityImpersonation;

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
            NtObject::<u64>::decrement_ref_count(*current_token);
            //  its now being referenced by another process. we need to increase its reference count
            NtObject::<u64>::increment_ref_count(token);
        }

        unsafe {
            current_token.write(token);
        }
    }

    pub fn kill(self, exit_code: NtStatus) -> Result<(), NtStatus> {
        match unsafe { PspTerminateThread(self.nt_thread as _, exit_code, 1) } {
            NtStatus::Success => Ok(()),
            err => Err(err),
        }
    }
}
