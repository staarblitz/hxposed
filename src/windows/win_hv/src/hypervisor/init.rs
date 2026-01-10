use alloc::boxed::Box;
use hv::SharedHostData;
use wdk_sys::ntddk::ExAllocatePool2;
use wdk_sys::POOL_FLAG_NON_PAGED;
use crate::hypervisor::ops;
use crate::hypervisor::vmexit::vmcall_handler;

pub(crate) fn init_hypervisor() {
    log::info!("Allocating memory for the hypervisor...");

    let mem = unsafe {
        ExAllocatePool2(
            POOL_FLAG_NON_PAGED,
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