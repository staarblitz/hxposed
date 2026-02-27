use crate::hypervisor::ops;
use crate::hypervisor::vmexit::vmcall_handler;
use crate::utils::intrin;
use crate::win::{ExAllocatePool2, PoolFlags};
use alloc::boxed::Box;
use core::arch::asm;
use hv::hypervisor::interrupt_handlers::{asm_interrupt_handler0, InterruptDescriptorTableEntry};
use hv::{InterruptDescriptorTable, SharedHostData};
use x86::dtables::{sidt, DescriptorTablePointer};

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
    hijack_idt();

    hv::virtualize_system(host_data);
}

// NOT PATCHGUARD COMPATIBLE
// TODO: move to nt module?
pub(crate) fn hijack_idt() {
    hv::platform_ops::get().run_on_all_processors(|_| {
        let mut table = DescriptorTablePointer::<u64>::default();
        unsafe { sidt(&mut table) };

        unsafe {
            // unset WP
            asm!(
            "mov rax, cr0",
            "btr rax, 16",
            "mov cr0, rax"
            )
        }
        let gp_entry = unsafe { (table.base as *mut InterruptDescriptorTableEntry).offset(13) };
        unsafe {
            intrin::ORIGINAL_GP_HANDLER = gp_entry.read().get_base();
            gp_entry.write(InterruptDescriptorTableEntry::new(
                intrin::hx_gp_handler as _,
                x86::segmentation::cs(),
            ));
        }
    });
}
