use core::mem;
use hv::platform_ops::PlatformOps;
use wdk_sys::ntddk::KeIpiGenericCall;
use wdk_sys::{ULONG_PTR, ntddk::MmGetPhysicalAddress};

pub(crate) struct WindowsOps;

impl PlatformOps for WindowsOps {
    fn run_on_all_processors(&self, callback: fn()) {
        unsafe {
            KeIpiGenericCall(Some(ipi_broadcast_worker), callback as _);
        }
    }

    fn pa(&self, va: *const core::ffi::c_void) -> u64 {
        #[expect(clippy::cast_sign_loss)]
        unsafe {
            MmGetPhysicalAddress(va.cast_mut()).QuadPart as u64
        }
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn ipi_broadcast_worker(argument: ULONG_PTR) -> ULONG_PTR {
    let callback: fn() = mem::transmute(argument);
    callback();

    ULONG_PTR::default()
}
