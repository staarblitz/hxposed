use crate::pe::map::get_nt_headers;
use crate::utils::get_fs;
use crate::{EFI_DRIVER_ALLOCATION, EFI_DRIVER_SIZE};
use core::ptr::copy_nonoverlapping;
use core::sync::atomic::Ordering;
use uefi::boot::{AllocateType, MemoryType};
use uefi::{Status, boot, cstr16};

pub struct HxPosed;

impl HxPosed {
    pub fn load() -> uefi::Result<()> {
        let mut fs = get_fs()?;

        log::info!("Reading HxPosed to memory...");

        let read = match fs.read(cstr16!(r"EFI\Staarblitz\HxPosed.sys")) {
            Ok(read) => read,
            Err(err) => {
                log::error!("Failed to read hxposed.sys into memory: {}", err);
                return Err(Status::ACCESS_DENIED.into());
            }
        };

        log::info!("HxPosed image size: {:x}", read.len() as u64);

        let nt_headers = unsafe { &*get_nt_headers(read.as_ptr()) };
        EFI_DRIVER_SIZE.store(
            nt_headers.OptionalHeader.SizeOfImage as _,
            Ordering::Relaxed,
        );

        log::info!(
            "HxPosed PE size: {:x}",
            nt_headers.OptionalHeader.SizeOfImage as u64
        );

        let alloc_count = (read.len() >> 12) + usize::from((read.len() & 0xff) != 0) + 1;
        log::info!("Allocating pages for HxPosed: {:x}", alloc_count);

        let pages = boot::allocate_pages(
            AllocateType::AnyPages,
            MemoryType::RUNTIME_SERVICES_CODE,
            alloc_count,
        )?;

        log::info!("HxPosed EFI base: {:x}", pages.as_ptr() as u64);

        EFI_DRIVER_ALLOCATION.store(pages.as_ptr(), Ordering::Relaxed);

        log::info!("Copying HxPosed to EFI allocation...");
        unsafe {
            copy_nonoverlapping(read.as_ptr(), pages.as_ptr(), read.len());
        }

        Ok(())
    }
}
