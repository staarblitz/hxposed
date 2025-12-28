use crate::nt::{BlImgAllocateImageBufferType, OslFwpKernelSetupPhase1Type, BL_IMG_ALLOCATE_IMAGE_BUFFER_PATTERN, OSL_FWP_KERNEL_SETUP_PHASE1_PATTERN};
use crate::{BL_IMG_ALLOCATE_IMAGE_BUFFER_DETOUR, OSL_FWP_KERNEL_SETUP_PHASE1_DETOUR};
use crate::pe::hooks::{bl_img_allocate_image_buffer, osl_fwp_kernel_setup_phase1};
use crate::utils::scanner::Scanner;

pub struct Winload {
    base: *const u8,
    size: usize,
}

impl Winload {
    pub fn new(base: *const u8, size: usize) -> Self {
        log::info!("Winload at: {:x}, size {:x}", base as usize, size);
        Winload { base, size }
    }

    pub fn patch(&self) {
        log::info!("Patching Winload...");

        let osl_fwp_pos = match Scanner::pattern_scan(
            self.base,
            self.size as _,
            &OSL_FWP_KERNEL_SETUP_PHASE1_PATTERN,
        ) {
            None => {
                log::error!("Could not find OslFwpKernelSetupPhase1! Boot continues as normal");
                return;
            }
            Some(pos) => pos as *const OslFwpKernelSetupPhase1Type,
        };

        log::info!("OslFwpKernelSetupPhase1: {:x}", osl_fwp_pos.addr());

        let bl_img_pos = match Scanner::pattern_scan(
            self.base,
            self.size as _,
            &BL_IMG_ALLOCATE_IMAGE_BUFFER_PATTERN,
        ) {
            None => {
                log::error!("Could not find BlImgAllocateImageBuffer! Boot continues as normal");
                return;
            }
            Some(pos) => pos as *const BlImgAllocateImageBufferType,
        };

        log::info!("BlImgAllocateImageBuffer: {:x}", bl_img_pos.addr());

        {
            let mut detour = OSL_FWP_KERNEL_SETUP_PHASE1_DETOUR.lock();
            detour.init(osl_fwp_pos, osl_fwp_kernel_setup_phase1 as _);
            detour.detour();
        }

        {
            let mut detour = BL_IMG_ALLOCATE_IMAGE_BUFFER_DETOUR.lock();
            detour.init(bl_img_pos, bl_img_allocate_image_buffer as _);
            detour.detour();
        }

        log::info!("Functions hooked");
    }
}
