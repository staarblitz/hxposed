use crate::as_pvoid;
use crate::win::ZwQueryInformationProcess;
use crate::win::alloc::PoolAllocSized;
use wdk::dbg_break;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::_PROCESSINFOCLASS::ProcessImageFileName;
use wdk_sys::ntddk::{IoGetCurrentProcess, ObOpenObjectByPointer};
use wdk_sys::{
    _REG_CREATE_KEY_INFORMATION_V1, _REG_NOTIFY_CLASS, HANDLE, NTSTATUS, OBJ_KERNEL_HANDLE,
    PACCESS_STATE, PROCESS_ALL_ACCESS, PVOID, PsProcessType, REG_NOTIFY_CLASS, STATUS_SUCCESS,
    ULONG, UNICODE_STRING,
};

#[unsafe(no_mangle)]
pub(crate) extern "C" fn registry_callback(
    _callback_context: PVOID,
    argument1: PVOID,
    argument2: PVOID,
) -> NTSTATUS {
    let op = REG_NOTIFY_CLASS::from(argument1 as i32);

    match op {
        _REG_NOTIFY_CLASS::RegNtPreOpenKeyEx => {
            // Starting with Windows 7, the actual data structure passed in when the notify class is RegNtPreCreateKeyEx or
            // RegNtPreOpenKeyEx is the V1 version of this structure, REG_CREATE_KEY_INFORMATION_V1 or REG_OPEN_KEY_INFORMATION_V1, respectively.
            // https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nc-wdm-ex_callback_function
            //
            // since we only support Windows 11, it's safe to assume this is v1 of the structure.
            let op_info = unsafe { &mut *(argument2 as *mut _REG_CREATE_KEY_INFORMATION_V1) };

            if op_info.CheckAccessMode == KernelMode as _ {
                return STATUS_SUCCESS; // we are not interested in kernel mode accesses.
            }

            dbg_break();

            let mut process = HANDLE::default();
            let status = unsafe {
                ObOpenObjectByPointer(
                    IoGetCurrentProcess() as _,
                    OBJ_KERNEL_HANDLE,
                    PACCESS_STATE::default(),
                    PROCESS_ALL_ACCESS,
                    *PsProcessType,
                    KernelMode as _,
                    &mut process,
                )
            };

            if status != STATUS_SUCCESS {
                return STATUS_SUCCESS; // let the registry manager handle this operation.
            }

            let mut return_length = ULONG::default();
            let _ = unsafe {
                ZwQueryInformationProcess(
                    process,
                    ProcessImageFileName,
                    PVOID::default(),
                    0,
                    &mut return_length,
                )
            };

            let mut proc_name = UNICODE_STRING::alloc_sized(return_length as usize);

            let status = unsafe {
                ZwQueryInformationProcess(
                    process,
                    ProcessImageFileName,
                    as_pvoid!(proc_name),
                    return_length,
                    &mut return_length,
                )
            };

            if status != STATUS_SUCCESS {
                return STATUS_SUCCESS; // let the registry manager handle this one.
            }
        }
        _ => {}
    }
    STATUS_SUCCESS
}
