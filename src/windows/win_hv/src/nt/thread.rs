use core::ptr::null_mut;
use wdk_sys::ntddk::{PsCreateSystemThread, ZwClose};
use wdk_sys::{HANDLE, NTSTATUS, PKSTART_ROUTINE, PVOID, STATUS_SUCCESS, THREAD_ALL_ACCESS};

pub struct NtThread;

impl NtThread {
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
}
