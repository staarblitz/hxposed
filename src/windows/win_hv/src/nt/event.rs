use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::utils::alloc::PoolAlloc;
use alloc::boxed::Box;
use wdk_sys::ntddk::{KeInitializeEvent, KeSetEvent, KeWaitForSingleObject, ObfDereferenceObject, ObfReferenceObject};
use wdk_sys::_EVENT_TYPE::SynchronizationEvent;
use wdk_sys::{HANDLE, KEVENT, LARGE_INTEGER, PKEVENT, STATUS_ALERTED, STATUS_SUCCESS, STATUS_TIMEOUT, STATUS_USER_APC, _KEVENT};
use wdk_sys::_KWAIT_REASON::Executive;
use wdk_sys::_MODE::KernelMode;

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
                ObfDereferenceObject(self.nt_event as _);
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
        let alertable = match alertable {
            true => 1,
            false => 0,
        };
        match unsafe {
            KeWaitForSingleObject(self.nt_event as _, Executive as _, KernelMode as _, alertable, &timeout as *const i64 as *const LARGE_INTEGER as _)
        } {
            STATUS_SUCCESS => WaitStatus::Signaled,
            STATUS_ALERTED => WaitStatus::Alerted,
            STATUS_USER_APC => WaitStatus::Alerted,
            STATUS_TIMEOUT => WaitStatus::TimedOut,
            _ => unreachable!()
        }
    }

    pub fn new() -> NtEvent {
        let me = Self {
            nt_event: Box::into_raw(_KEVENT::alloc()),
            owns: false,
            owns_alloc: true,
        };
        unsafe { KeInitializeEvent(me.nt_event, SynchronizationEvent as _, 0) };

        me
    }

    pub fn from_ptr(event: PKEVENT) -> NtEvent {
        Self::open_event(event, false)
    }

    pub fn from_handle(handle: HANDLE) -> Result<NtEvent, ()> {
        let process = NtProcess::current();
        let obj = match NtObject::<KEVENT>::from_handle(handle, process.get_handle_table()) {
            Ok(ptr) => ptr,
            Err(_) => return Err(()),
        };

        Ok(Self::open_event(obj.object_addr, true))
    }

    fn open_event(ptr: PKEVENT, owns: bool) -> Self {
        unsafe {
            ObfReferenceObject(ptr as _);
        }
        Self {
            nt_event: ptr,
            owns,
            owns_alloc: false,
        }
    }

    pub fn signal(&self) -> bool {
        unsafe { KeSetEvent(self.nt_event, 0, 0) == 0 }
    }
}
