#![no_main]
#![no_std]
extern crate alloc;

mod nt;
mod pe;
mod utils;

use crate::nt::bootmgfw::Bootmgfw;
use crate::nt::*;
use crate::pe::detour::Detour;
use crate::utils::hxposed::HxPosed;
use core::sync::atomic::{AtomicPtr, AtomicU64};
use spin::Mutex;
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;

pub static IMG_ARCH_START_BOOT_APPLICATION_DETOUR: Mutex<Detour<ImgArchStartBootApplicationType>> =
    Mutex::new(Detour::<ImgArchStartBootApplicationType>::default());
pub static OSL_FWP_KERNEL_SETUP_PHASE1_DETOUR: Mutex<Detour<OslFwpKernelSetupPhase1Type>> =
    Mutex::new(Detour::<OslFwpKernelSetupPhase1Type>::default());
pub static BL_IMG_ALLOCATE_IMAGE_BUFFER_DETOUR: Mutex<Detour<BlImgAllocateImageBufferType>> =
    Mutex::new(Detour::<BlImgAllocateImageBufferType>::default());

pub static NT_DRIVER_ALLOCATION: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
pub static EFI_DRIVER_ALLOCATION: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
pub static EFI_DRIVER_SIZE: AtomicU64 = AtomicU64::new(0);

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    com_logger::builder()
        .base(0x3f8)
        .filter(log::LevelFilter::Trace)
        .setup();
    log::set_max_level(log::LevelFilter::Trace);

    let proto = boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle()).unwrap();

    log::info!("Welcome to HxLoader!");
    log::info!("HxLoader's image base: {:x}", proto.info().0 as u64);
    log::info!("HxLoader's image size: {:x}", proto.info().1);

    drop(proto);

    // this is not critical.
    let _ = NtVars::disable_vbs();

    match HxPosed::load() {
        Ok(_) => {
            log::info!("HxPosed is loaded into memory.");
        }
        Err(err) => {
            log::error!("Failed to load hxposed into memory: {}", err);
            return err.status();
        }
    }

    let bootmgfw_img = match Bootmgfw::load() {
        Ok(img) => img,
        Err(err) => {
            log::error!("Failed to load bootmgfw: {}", err);
            return err.status();
        }
    };

    match bootmgfw_img.patch() {
        Ok(_) => {
            log::info!("Patched bootmgfw");
        }
        Err(err) => {
            log::error!("Failed to patch bootmgfw: {}", err);

            return err.status();
        }
    };

    match bootmgfw_img.start() {
        Ok(_) => {}
        Err(err) => {
            log::error!("Failed to start bootmgfw: {}", err);

            return err.status();
        }
    };

    Status::SUCCESS
}
