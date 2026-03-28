use crate::win::{
    GROUP_AFFINITY, KeGetProcessorNumberFromIndex, KeQueryActiveProcessorCountEx,
    KeRevertToUserGroupAffinityThread, KeSetSystemGroupAffinityThread,
    PROCESSOR_NUMBER,
};

pub(crate) struct PlatformOps;

impl PlatformOps {
    pub fn run_on_all_processors(mut callback: impl FnMut(u32)) {
        for index in 0..unsafe { KeQueryActiveProcessorCountEx(0xffff) } {
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
}