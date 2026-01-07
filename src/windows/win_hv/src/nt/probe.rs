use wdk_sys::ntddk::{ProbeForRead, ProbeForWrite};

pub(crate) fn probe_for_write(address: *mut u8, length: usize, align: u32) -> Result<(), ()> {
    unsafe {
        match microseh::try_seh(|| ProbeForWrite(address as _, length as _, align)) {
            Ok(()) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

pub(crate) fn probe_for_read(address: *mut u8, length: usize, align: u32) -> Result<(), ()> {
    unsafe {
        match microseh::try_seh(|| ProbeForRead(address as _, length as _, align)) {
            Ok(()) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
