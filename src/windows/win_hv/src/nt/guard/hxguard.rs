use crate::nt::process::NtProcess;
use crate::utils::alloc::PoolAllocSized;
use crate::win::utf_to_unicode::Utf8ToUnicodeString;
use crate::win::utils::{_RtlDuplicateUnicodeString, RtlBufferContainsBuffer};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::*;
use wdk_sys::*;

/// Protects HxPosed from unauthorized access
pub struct HxGuard {
    pub registry_guard_enabled: bool,
    registry_callback_cookie: u64,
}

impl HxGuard {
    pub const fn new(registry_guard_enabled: bool) -> Self {
        Self {
            registry_guard_enabled,
            registry_callback_cookie: 0,
        }
    }

    pub fn init(&mut self) {
        log::info!("Initializing HxGuard");
        if self.registry_guard_enabled {
            log::info!("Registry Guard is enabled.");
            match unsafe {
                CmRegisterCallback(
                    Some(Self::registry_callback),
                    PVOID::default(),
                    &mut self.registry_callback_cookie as *mut _ as _,
                )
            } {
                STATUS_SUCCESS => {
                    log::info!("Registry Callbacks are being watched by HxGuard.")
                }
                err => {
                    panic!("Error registering registry callbacks: {:?}", err);
                }
            }
        }
    }

    ///
    /// # Registry Callback
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
    extern "C" fn registry_callback(
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

                if op_info.CheckAccessMode as u32 == KernelMode as u32 {
                    return STATUS_SUCCESS; // we are not interested in kernel mode accesses.
                }

                let mut ret_len = 0;
                let _ = unsafe {
                    ObQueryNameString(
                        op_info.RootObject,
                        POBJECT_NAME_INFORMATION::default(),
                        0,
                        &mut ret_len,
                    )
                };

                let mut alloc = OBJECT_NAME_INFORMATION::alloc_sized(ret_len as _);

                // get full registry path
                match unsafe {
                    ObQueryNameString(op_info.RootObject, alloc.as_mut(), ret_len, &mut ret_len)
                } {
                    STATUS_SUCCESS => {},
                    err => {
                        log::warn!("Failed to query object name: {:x}", err);
                        return STATUS_SUCCESS;
                    }
                }

                // UNICODE_STRING boilerplate to get absolute path for the registry notification

                let mut dup = unsafe { _RtlDuplicateUnicodeString(&mut alloc.as_mut().Name, 256) };

                let _ = unsafe {
                    RtlAppendUnicodeStringToString(dup.as_mut(), "\\".to_unicode_string().as_mut())
                };
                let _ = unsafe { RtlAppendUnicodeStringToString(dup.as_mut(), op_info.RemainingName) };

                let path = "SOFTWARE\\HxPosed".to_unicode_string();

                // RtlCompareUnicodeString won't work, because "target" is inside "source", but unknown where.
                let result = unsafe {
                    RtlBufferContainsBuffer(
                        dup.as_ref().Buffer as _,
                        dup.as_ref().Length as _,
                        path.as_ref().Buffer as _,
                        path.as_ref().Length as _,
                    )
                };

                if !result {
                    return STATUS_SUCCESS;
                }

                let process = NtProcess::current();

                // we need to convert this to a DOS path. Otherwise, it will be vulnerable to attacks from other drives. like F:\Program Files\\HxPosed\\HxPosed.GUI.exe
                let path = "HxPosed.GUI.exe".to_unicode_string();

                let process_path = &unsafe { *process.nt_path };

                let result = unsafe {
                    RtlBufferContainsBuffer(
                        process_path.Buffer as _,
                        process_path.Length as _,
                        path.as_ref().Buffer as _,
                        path.as_ref().Length as _,
                    )
                };

                return if result {
                    STATUS_SUCCESS
                } else {
                    STATUS_ACCESS_DENIED
                };
            }
            _ => {}
        }
        STATUS_SUCCESS
    }

}
