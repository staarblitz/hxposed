//! This module implements the platform agnostic hypervisor core.

#[cfg(not(test))]
pub mod allocator;
mod amd;
mod apic_id;
pub mod gdt_tss;
pub mod host;
mod intel;
pub mod interrupt_handlers;
pub mod paging_structures;
pub mod panic;
pub mod platform_ops;
mod registers;
mod segment;
mod support;
mod switch_stack;
mod x86_instructions;

extern crate hxposed_core;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use bit_field::BitField;
use hxposed_core::hxposed::call::HypervisorCall;
use spin::Once;
use x86::cpuid::cpuid;

use self::interrupt_handlers::InterruptDescriptorTable;
use crate::hypervisor::host::{Guest};
use crate::{GdtTss, PagingStructures, hypervisor::registers::Registers};

pub static PROCESSOR_BITMASK: AtomicU64 = AtomicU64::new(0);

/// Hyperjacks the current system by virtualizing all logical processors on this
/// system.
pub fn virtualize_system(shared_host: SharedHostData) {
    log::info!("Virtualizing the all processors");

    apic_id::init();
    let _ = SHARED_HOST_DATA.call_once(|| shared_host);

    // Virtualize each logical processor.
    platform_ops::get().run_on_all_processors(|index| {
        // Take a snapshot of current register values. This will be the initial
        // state of the guest _including RIP_. This means that the guest starts execution
        // right after this function call. Think of it as the setjmp() C standard
        // function.
        let registers = Registers::capture_current();

        // In the first run, our hypervisor is not installed and the branch is
        // taken. After starting the guest, the second run, the hypervisor is already
        // installed and we will bail out.

        let mut mask = PROCESSOR_BITMASK.load(Ordering::Acquire);
        if !mask.get_bit(index as _) {
            PROCESSOR_BITMASK.store(*mask.set_bit(index as _, true), Ordering::Release);

            log::info!("Virtualizing processor: {}", index);

            // We are about to execute host code with newly allocated stack.
            // This is required because the guest will start executing with the
            // current stack. If we do not change the stack for the host, as soon
            // as the guest starts, it will smash host's stack.
            switch_stack::jump_with_new_stack(host::main, &registers);
        }
        log::info!("Virtualized processor: {}", index);
    });

    log::info!("Virtualized the all processors");
}

/// A collection of data that the host depends on for its entire lifespan.
#[derive(Debug, Default)]
pub struct SharedHostData {
    /// The paging structures for the host. If `None`, the current paging
    /// structure is used for both the host and the guest.
    pub pt: Option<PagingStructures>,

    /// The IDT for the host. If `None`, the current IDT is used for both the
    /// host and the guest.
    pub idt: Option<InterruptDescriptorTable>,

    /// The GDT and TSS for the host for each logical processor. If `None`,
    /// the current GDTs and TSSes are used for both the host and the guest.
    pub gdts: Option<Vec<GdtTss>>,

    pub hv_cpuid_eax: u32,

    pub vmcall_handler: Option<fn(&mut dyn Guest, HypervisorCall) -> bool>,
}

static SHARED_HOST_DATA: Once<SharedHostData> = Once::new();

const HV_CPUID_VENDOR_AND_MAX_FUNCTIONS: u32 = 0x4000_0000;
pub const HV_CPUID_INTERFACE: u32 = 0x4000_0001;