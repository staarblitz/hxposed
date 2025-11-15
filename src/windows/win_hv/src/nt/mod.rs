use core::sync::atomic::{AtomicU64, Ordering};
use wdk_sys::ntddk::RtlGetVersion;
use wdk_sys::{PEPROCESS, RTL_OSVERSIONINFOW};

pub(crate) mod bind;

pub(crate) static NT_BUILD: AtomicU64 = AtomicU64::new(0);

pub(crate) fn get_nt_info() {
    let mut info = RTL_OSVERSIONINFOW::default();
    unsafe { RtlGetVersion(&mut info) };

    NT_BUILD.store(info.dwBuildNumber as _, Ordering::Relaxed);
}