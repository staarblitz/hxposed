use crate::nt::{OslFwpKernelSetupPhase1Type, KE_INIT_SPECIFIC_STATE_PATTERN, KI_IS_NX_SUPPORTED};
use crate::utils::scanner::Scanner;

pub struct NtOsKrnl{}

impl NtOsKrnl {
    pub fn disable_kpp(base: *const u8, size: usize) -> Result<(), ()> {
        let ke_init_specific_state_pos = match Scanner::pattern_scan(base, size, &KE_INIT_SPECIFIC_STATE_PATTERN) {
            None => {
                log::error!("Could not find KeInitAmd64SpecificState pattern! Boot continues as normal");
                return Err(());
            }
            Some(pos) => pos as *mut u8,
        };

        // i dont remember exactly why i was disabling this. but it should be relevant to patchguard
        let ki_is_nx_supported_pos = match Scanner::pattern_scan(base, size, &KI_IS_NX_SUPPORTED) {
            None => {
                log::error!("Could not find KiIsNxSupported pattern! Boot continues as normal");
                return Err(());
            }
            Some(pos) => pos as *mut u8,
        };

        log::info!("KeInitAmd64SpecificState: {:x}", ke_init_specific_state_pos.addr());
        log::info!("KiIsNxSupported: {:x}", ki_is_nx_supported_pos.addr());

        unsafe {
            ke_init_specific_state_pos.write_volatile(0xEB); // jmp
            ki_is_nx_supported_pos.write_volatile(0xEB); // jmp as well
        }

        log::info!("PatchGuard has been patched");

        Ok(())
    }
}