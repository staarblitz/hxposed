use crate::nt::process::NtProcess;
use crate::nt::registry::NtKey;
use crate::utils::alloc::PoolAllocSized;
use crate::win::unicode_string::{self, UnicodeString};
use alloc::string::ToString;
use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi64, _mm_loadu_si128, _mm_movemask_pd, _mm_set1_epi64x,
};
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::*;
use wdk_sys::*;
use crate::HX_GUARD;

pub static mut VALID_CALLERS: [u64; 256] = [0; 256];
//pub static VALID_CALLER_COUNT: AtomicU32 = AtomicU32::new(0);

pub enum RegistryProtection {
    Initialized(u64),
    Enabled,
    Disabled,
}

/// Protects HxPosed from unauthorized access
pub struct HxGuard {
    pub registry_protection: RegistryProtection,
    pub caller_verification: bool,
}

impl HxGuard {
    pub const fn new() -> Self {
        Self {
            registry_protection: RegistryProtection::Disabled,
            caller_verification: false,
        }
    }

    // not my code
    #[allow(static_mut_refs)]
    pub fn is_valid_caller(hash: u64) -> bool {
        unsafe {
            if !HX_GUARD.caller_verification {
                return true;
            }

            let mut i = 0;
            let len = VALID_CALLERS.len();

            let needle_vec = _mm_set1_epi64x(hash as i64);

            while i + 2 <= len {
                let ptr = VALID_CALLERS.as_ptr().add(i) as *const __m128i;
                let chunk = _mm_loadu_si128(ptr);

                let cmp = _mm_cmpeq_epi64(chunk, needle_vec);
                let mask = _mm_movemask_pd(core::mem::transmute(cmp));

                if mask != 0 {
                    return true;
                }

                i += 2;
            }

            false
        }
    }

    #[allow(static_mut_refs)]
    pub fn init(&mut self) {
        log::info!("Initializing HxGuard");
        let key = match NtKey::open("\\Registry\\Machine\\SOFTWARE\\HxPosed\\HxGuard") {
            Ok(x) => x,
            Err(err) => {
                panic!("Failed to open HxPosed key: {:x}", err)
            }
        };

        self.caller_verification = match key.get_value::<BOOLEAN>("CallerVerification") {
            Ok(x) => *x == 1,
            Err(err) => {
                log::warn!(
                    "Failed to read CallerVerification. Reverting to default (TRUE): {:x}",
                    err
                );
                true
            }
        };

        self.registry_protection = match key.get_value::<BOOLEAN>("RegistryProtection") {
            Ok(x) => match x {
                1 => RegistryProtection::Enabled,
                0 => RegistryProtection::Disabled,
                _ => unreachable!(),
            },
            Err(err) => {
                log::warn!(
                    "Failed to read RegistryProtection. Reverting to default (TRUE): {:x}",
                    err
                );
                RegistryProtection::Enabled
            }
        };

        if self.caller_verification {
            log::info!("Caller verification is enabled.");

            let key = match NtKey::open(
                "\\Registry\\Machine\\SOFTWARE\\HxPosed\\HxGuard\\CallerVerification",
            ) {
                Ok(x) => x,
                Err(err) => {
                    panic!("Failed to open CallerVerification key: {:x}", err)
                }
            };

            let mut default: [u64; 256] = [0; 256];
            let values = match key.get_value::<[u64; 256]>("VerifiedCallers") {
                Ok(x) => x,
                Err(err) => {
                    log::error!(
                        "Failed to read VerifiedCallers key! No caller is allowed. {:x}",
                        err
                    );
                    &mut default
                }
            };

            // SAFETY: no one touches VALID_CALLERS yet.
            unsafe {
                core::ptr::copy_nonoverlapping::<u64>(
                    values.as_ptr(),
                    VALID_CALLERS.as_mut_ptr(),
                    values.len(),
                );
            }

            log::info!("Caller verification is active");
        }
        match self.registry_protection {
            RegistryProtection::Enabled => {
                let mut cookie = UINT64::default();
                log::info!("Registry protection is enabled.");
                match unsafe {
                    CmRegisterCallback(
                        Some(Self::registry_callback),
                        PVOID::default(),
                        &mut cookie as *mut _ as _,
                    )
                } {
                    STATUS_SUCCESS => {
                        log::info!("Registry callbacks are being watched by HxGuard.")
                    }
                    err => {
                        panic!("Error registering registry callbacks: {:?}", err);
                    }
                }
                self.registry_protection = RegistryProtection::Initialized(cookie);
            }
            RegistryProtection::Disabled => {}
            _ => unreachable!(),
        };
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
                    STATUS_SUCCESS => {}
                    err => {
                        log::warn!("Failed to query object name: {:x}", err);
                        return STATUS_SUCCESS;
                    }
                }

                let remaining_name =
                    UnicodeString::from_unicode_string(unsafe { &*op_info.RemainingName });
                let mut path = UnicodeString::from_unicode_string(&alloc.Name);
                path.concat("\\");
                path.concat_from_unicode_string(&remaining_name);

                if path.contains("HxPosed") {
                    let process = NtProcess::current();

                    let process_path = process.get_nt_path();

                    return if !process_path.contains("HxPosed.GUI.exe") {
                        STATUS_ACCESS_DENIED
                    } else {
                        STATUS_SUCCESS
                    };
                }

                return STATUS_SUCCESS;
            }
            _ => {}
        }
        STATUS_SUCCESS
    }
}
