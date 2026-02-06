use crate::win::{KeGetProcessorNumberFromIndex, KeQueryActiveProcessorCountEx, KeRevertToUserGroupAffinityThread, KeSetSystemGroupAffinityThread, MmGetPhysicalAddress, GROUP_AFFINITY, PROCESSOR_NUMBER};
use hv::platform_ops::PlatformOps;

pub(crate) struct WindowsOps;

impl PlatformOps for WindowsOps {
    fn run_on_all_processors(&self, callback: fn(index: u32)) {
        for index in 0.. unsafe { KeQueryActiveProcessorCountEx(0xffff) } {
            let mut processor_number = PROCESSOR_NUMBER::default();
            let _ = unsafe { KeGetProcessorNumberFromIndex(index, &raw mut processor_number) };

            let mut old_affinity = GROUP_AFFINITY::default();
            let mut affinity = GROUP_AFFINITY {
                Group: processor_number.Group,
                Mask: 1 << processor_number.Number,
                Reserved: [0, 0, 0],
            };
            unsafe { KeSetSystemGroupAffinityThread(&raw mut affinity, &raw mut old_affinity) };

            callback(index);

            unsafe { KeRevertToUserGroupAffinityThread(&raw mut old_affinity) };
        }
    }

    fn pa(&self, va: *const core::ffi::c_void) -> u64 {
        #[expect(clippy::cast_sign_loss)]
        unsafe {
            MmGetPhysicalAddress(va.cast_mut())
        }
    }
}
