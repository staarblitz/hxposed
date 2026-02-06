use crate::HX_GUARD;
use crate::nt::process::NtProcess;
use crate::nt::registry::NtKey;
use crate::utils::alloc::PoolAllocSized;
use crate::win::unicode_string::{self, UnicodeString};
use crate::win::{Boolean, NtStatus, PVOID};
use alloc::string::ToString;
use core::arch::x86_64::{
    __m128i, _mm_cmpeq_epi64, _mm_loadu_si128, _mm_movemask_pd, _mm_set1_epi64x,
};
use core::ptr::null_mut;

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
                log::error!("Failed to open HxPosed key: {:x}", err);
                return;
            }
        };

        self.caller_verification = match key.get_value::<Boolean>("CallerVerification") {
            Ok(x) => *x == Boolean::True,
            Err(err) => {
                log::warn!(
                    "Failed to read CallerVerification. Reverting to default (TRUE): {:x}",
                    err
                );
                true
            }
        };

        self.registry_protection = match key.get_value::<Boolean>("RegistryProtection") {
            Ok(x) => match x {
                Boolean::True => RegistryProtection::Enabled,
                Boolean::False => RegistryProtection::Disabled,
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
                log::warn!("Registry protection is not implemented!")
            }
            RegistryProtection::Disabled => {}
            _ => unreachable!(),
        };
    }
}
