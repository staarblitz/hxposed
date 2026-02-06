use crate::hypervisor::ops;
use crate::hypervisor::vmexit::vmcall_handler;
use crate::win::{ExAllocatePool2, PoolFlags};
use alloc::boxed::Box;
use hv::SharedHostData;

pub(crate) fn init_hypervisor() {
    log::info!("Allocating memory for the hypervisor...");

    let mem = unsafe {
        ExAllocatePool2(
            PoolFlags::NonPaged,
            hv::allocator::ALLOCATION_BYTES as _,
            0x2009,
        )
    };

    hv::allocator::init(mem as _);

    hv::platform_ops::init(Box::new(ops::WindowsOps));

    // TODO: use custom gdt and so on for more security?
    let mut host_data = SharedHostData::default();
    host_data.vmcall_handler = Some(vmcall_handler);

    hv::virtualize_system(host_data);
}
