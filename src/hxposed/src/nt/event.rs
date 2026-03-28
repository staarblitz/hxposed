use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::utils::timing;
use crate::win::{
    Boolean, EventType, HANDLE, KeInitializeEvent, KeSetEvent, KeWaitForSingleObject, NtStatus,
    PKEVENT, ProcessorMode, WaitReason,
};
use alloc::boxed::Box;
use crate::nt::{get_object_header, ObjectBody};

pub struct NtEvent {
    pub nt_event: PKEVENT,
    pub owns: bool,
    pub owns_alloc: bool,
}

unsafe impl Send for NtEvent {}
unsafe impl Sync for NtEvent {}

impl Drop for NtEvent {
    fn drop(&mut self) {
        if self.owns {
            unsafe {
                NtObject::decrement_handle_count_raw(self.nt_event);
            }
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum WaitStatus {
    TimedOut,
    Alerted,
    Signaled,
}

impl NtEvent {
    pub fn wait(&self, alertable: bool, timeout: i64) -> WaitStatus {
        let mut timeout = timing::relative(timing::milliseconds(timeout));
        match unsafe {
            KeWaitForSingleObject(
                self.nt_event as _,
                WaitReason::Executive,
                ProcessorMode::KernelMode,
                Boolean::from(alertable),
                &mut timeout,
            )
        } {
            NtStatus::Success => WaitStatus::Signaled,
            NtStatus::Alerted | NtStatus::UserApc => WaitStatus::Alerted,
            NtStatus::TimeOut => WaitStatus::TimedOut,
            _ => unreachable!(),
        }
    }

    pub fn new() -> NtEvent {
        let me = Self {
            nt_event: Box::into_raw(Box::new([0; 24])) as _,
            owns: false,
            owns_alloc: true,
        };
        unsafe { KeInitializeEvent(me.nt_event, EventType::SynchronizationEvent, Boolean::False) };

        me
    }

    pub fn from_ptr(event: PKEVENT) -> NtEvent {
        Self::open_event(event, false)
    }

    pub fn from_handle(handle: HANDLE) -> Result<NtEvent, ()> {
        let process = NtProcess::current();
        let obj = match NtObject::from_handle(handle, process.get_handle_table()) {
            Some(ptr) => ptr,
            None => return Err(()),
        };

        Ok(Self::open_event(obj.object_addr.0 as _, true))
    }

    fn open_event(ptr: PKEVENT, owns: bool) -> Self {
        unsafe {
            NtObject::decrement_ref_count_raw(ptr);
        }
        Self {
            nt_event: ptr,
            owns,
            owns_alloc: false,
        }
    }

    pub fn signal(&self) -> bool {
        unsafe { KeSetEvent(self.nt_event, 0, Boolean::False) == 0 }
    }
}
