use wdk_sys::ntddk::{ProbeForRead, ProbeForWrite};

pub(crate) fn probe_for_write(address: *mut u8, length: usize) -> Result<(), microseh::Exception> {
    unsafe {
        match microseh::try_seh(|| ProbeForWrite(address as _, length as _, 8)) {
            Ok(()) => Ok(()),
            Err(x) => Err(x),
        }
    }
}

pub(crate) fn probe_for_read(address: *mut u8, length: usize) -> Result<(), ()> {
    unsafe {
        match microseh::try_seh(|| ProbeForRead(address as _, length as _, 8)) {
            Ok(()) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
