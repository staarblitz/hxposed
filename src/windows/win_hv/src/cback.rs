use crate::win::alloc::PoolAllocSized;
use crate::win::{RtlUnicodeStringContainsUnicodeString, Utf8ToUnicodeString, _RtlDuplicateUnicodeString};
use wdk::{dbg_break, println};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{IoGetCurrentProcess, ObQueryNameString, RtlAppendUnicodeStringToString, RtlDuplicateUnicodeString};
use wdk_sys::{
    _REG_CREATE_KEY_INFORMATION_V1, _REG_NOTIFY_CLASS, FALSE, KEY_ALL_ACCESS, NTSTATUS,
    OBJECT_NAME_INFORMATION, PCUNICODE_STRING, POBJECT_NAME_INFORMATION, PVOID, REG_NOTIFY_CLASS,
    STATUS_SUCCESS,
};

///
/// # Registry Callback (work in progress)
///
/// This function serves as a registry filter to stop smart people from accessing to HxPosed registry key.
///
/// ## Arguments
/// Described in PEX_CALLBACK_FUNCTION.
///
/// First argument is unused.
///
/// ## Return
/// Visit [MSDN article about this](https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/handling-notifications)
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

            // as u32 to avoid intellij bullshit
            if op_info.CheckAccessMode as u32 == KernelMode as u32 {
                return STATUS_SUCCESS; // we are not interested in kernel mode accesses.
            }

            dbg_break();

            let mut ret_len = 0;
            unsafe {
                ObQueryNameString(
                    op_info.RootObject,
                    POBJECT_NAME_INFORMATION::default(),
                    0,
                    &mut ret_len,
                )
            };

            let mut alloc = OBJECT_NAME_INFORMATION::alloc_sized(ret_len as _);

            let status = unsafe {
                ObQueryNameString(op_info.RootObject, alloc.as_mut(), ret_len, &mut ret_len)
            };

            if status != STATUS_SUCCESS {
                println!("Failed to query object name");
                return STATUS_SUCCESS;
            }

            let mut dup = unsafe{_RtlDuplicateUnicodeString(&mut alloc.as_mut().Name, 256)};

            unsafe{
                RtlAppendUnicodeStringToString(dup.as_mut(), op_info.RemainingName)
            }

            let result = unsafe {
                RtlUnicodeStringContainsUnicodeString(
                    dup.as_ref(),
                    "HxPosed".to_unicode_string().as_ref(),
                    FALSE as _,
                )
            } == 1;

            if !result {
                return STATUS_SUCCESS;
            }

            // get the juicy _EPROCESS;
            let process =
                unsafe { IoGetCurrentProcess() } as *mut crate::nt::bind::w25h2::_EPROCESS;
            let process = &mut unsafe { *(process) };
            let result = unsafe {
                RtlUnicodeStringContainsUnicodeString(
                    process.SeAuditProcessCreationInfo.ImageFileName as PCUNICODE_STRING,
                    "HxPosed.GUI.exe".to_unicode_string().as_ref(),
                    FALSE as _,
                )
            } == 1;

            // if it was the HxPosed manager that opened this key, allow all access. No access if it wasn't.
            op_info.GrantedAccess = if result { KEY_ALL_ACCESS } else { 0 }
        }
        _ => {}
    }
    STATUS_SUCCESS
}
