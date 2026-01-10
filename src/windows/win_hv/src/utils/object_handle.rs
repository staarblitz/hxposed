use crate::utils::handlebox::HandleBox;
use wdk_sys::ntddk::ObOpenObjectByPointer;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::{
    PsProcessType, PsThreadType, ACCESS_MASK, HANDLE, NTSTATUS, PEPROCESS, PETHREAD, POBJECT_TYPE,
    PROCESS_ALL_ACCESS, PVOID, STATUS_SUCCESS, THREAD_ALL_ACCESS,
};

pub trait OpenHandle {
    fn get_handle(&self) -> Result<HandleBox, NTSTATUS>;
}

impl OpenHandle for PEPROCESS {
    fn get_handle(&self) -> Result<HandleBox, NTSTATUS> {
        unsafe { get_handle_for_object(*self as _, *PsProcessType, PROCESS_ALL_ACCESS) }
    }
}

impl OpenHandle for PETHREAD {
    fn get_handle(&self) -> Result<HandleBox, NTSTATUS> {
        unsafe { get_handle_for_object(*self as _, *PsThreadType, THREAD_ALL_ACCESS) }
    }
}

fn get_handle_for_object(
    object: PVOID,
    obj_type: POBJECT_TYPE,
    rights: ACCESS_MASK,
) -> Result<HandleBox, NTSTATUS> {
    let mut handle = HANDLE::default();
    match unsafe {
        ObOpenObjectByPointer(
            object,
            0,
            Default::default(),
            rights,
            obj_type,
            KernelMode as _,
            &mut handle,
        )
    } {
        STATUS_SUCCESS => Ok(HandleBox::new(handle)),
        err => Err(err),
    }
}
