use hv::platform_ops::PlatformOps;
use wdk_sys::ntddk::{
    KeGetProcessorNumberFromIndex, KeQueryActiveProcessorCountEx,
    KeRevertToUserGroupAffinityThread, KeSetSystemGroupAffinityThread,
};
use wdk_sys::{
    ntddk::MmGetPhysicalAddress, ALL_PROCESSOR_GROUPS, GROUP_AFFINITY, NT_SUCCESS,
    PROCESSOR_NUMBER,
};

pub(crate) struct WindowsOps;

impl PlatformOps for WindowsOps {
    fn run_on_all_processors(&self, callback: fn()) {
        fn processor_count() -> u32 {
            unsafe { KeQueryActiveProcessorCountEx(u16::try_from(ALL_PROCESSOR_GROUPS).unwrap()) }
        }

        for index in 0..processor_count() {
            let mut processor_number = PROCESSOR_NUMBER::default();
            let status = unsafe { KeGetProcessorNumberFromIndex(index, &raw mut processor_number) };
            assert!(NT_SUCCESS(status));

            let mut old_affinity = GROUP_AFFINITY::default();
            let mut affinity = GROUP_AFFINITY {
                Group: processor_number.Group,
                Mask: 1 << processor_number.Number,
                Reserved: [0, 0, 0],
            };
            unsafe { KeSetSystemGroupAffinityThread(&raw mut affinity, &raw mut old_affinity) };

            callback();

            unsafe { KeRevertToUserGroupAffinityThread(&raw mut old_affinity) };
        }
    }

    fn pa(&self, va: *const core::ffi::c_void) -> u64 {
        #[expect(clippy::cast_sign_loss)]
        unsafe {
            MmGetPhysicalAddress(va.cast_mut()).QuadPart as u64
        }
    }
}
