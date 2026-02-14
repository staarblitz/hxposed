use bit_field::BitField;
use core::ffi::c_void;
use core::ptr::null_mut;
use uefi::boot::MemoryAttribute;
use uefi::fs::FileSystem;
use uefi::proto::security::MemoryProtection;
use uefi::proto::ProtocolPointer;
use uefi::{boot, Identify, Status};
use uefi::table::cfg::MemoryProtectionAttribute;

pub(crate) mod hxposed;
pub(crate) mod scanner;

pub unsafe fn protect_efi_mem(ptr: *mut c_void, attr: MemoryAttribute) {
    let mut protoptr: *mut c_void = null_mut();
    let proto = unsafe {
        match ((*(*uefi::table::system_table_raw().unwrap().as_ptr()).boot_services)
            .locate_protocol)(&MemoryProtection::GUID, null_mut(), &mut protoptr)
        {
            Status::SUCCESS => {}
            _ => {
                log::error!("Memory attribute protocol is not supported.");
                return;
            }
        }

        &mut *MemoryProtection::mut_ptr_from_ffi(protoptr)
    };
    let new_ptr = (ptr.addr() & !0xFFF) as u64;
    let range = new_ptr..(new_ptr + 4096);
    match proto.clear_memory_attributes(range.clone(), MemoryAttribute::EXECUTE_PROTECT | MemoryAttribute::WRITE_PROTECT) {
        Ok(_) => {}
        Err(x) => {
            log::error!("Failed to clear memory attributes: {:?}", x);
        }
    };
    match proto.set_memory_attributes(range, attr) {
        Ok(_) => {}
        Err(x) => {
            log::error!("Failed to set memory attributes: {:?}", x);
        }
    }
}

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
