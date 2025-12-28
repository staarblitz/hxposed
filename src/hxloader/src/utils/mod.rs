use uefi::boot;
use uefi::fs::FileSystem;

pub(crate) mod hxposed;
pub(crate) mod scanner;

// make the life easier
pub fn get_fs() -> uefi::Result<FileSystem> {
    match boot::get_image_file_system(boot::image_handle()) {
        Ok(fs) => Ok(FileSystem::new(fs)),
        Err(err) => {
            log::error!("Failed to open file system: {}", err.status());

            Err(err)
        }
    }
}

pub unsafe fn get_cstr_len(pointer: *const u8) -> usize {
    let mut tmp: u64 = pointer as u64;

    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }

    (tmp - pointer as u64) as _
}