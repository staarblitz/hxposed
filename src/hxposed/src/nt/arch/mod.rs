use crate::nt::arch::hxfs::HxFs;
use crate::nt::arch::idt::InterruptDescriptorTableRaw;
use crate::nt::arch::ops::PlatformOps;
use crate::utils::intrin::sidt;
use crate::win::{ExAllocatePool2, MmGetPhysicalAddress, MmGetVirtualForPhysical, PoolFlags};
use alloc::boxed::Box;
use alloc::vec::Vec;
use bit_field::BitField;
use core::arch::{asm, global_asm};
use core::ffi::c_void;
use x86::controlregs::{cr0, cr0_write, Cr0};
use x86::msr::{rdmsr, wrmsr, IA32_FS_BASE, IA32_GS_BASE, IA32_KERNEL_GSBASE, IA32_LSTAR};

pub(crate) mod cr3;
pub mod hxfs;
mod idt;
mod ops;
pub(crate) mod pt;

pub fn virt_to_phys(virt: u64) -> u64 {
    unsafe { MmGetPhysicalAddress(virt as _) }
}

pub fn phys_to_virt(phys: u64) -> u64 {
    unsafe { MmGetVirtualForPhysical(phys) as _ }
}

pub fn hijack_pcrs() {
    PlatformOps::run_on_all_processors(|_| {
        let mut cr0 = unsafe { cr0() };
        cr0.set(Cr0::CR0_WRITE_PROTECT, false);
        unsafe { cr0_write(cr0) };

        // now redirect #GP for our purpose
        // in NT, all cores have their own IDT. but they point to same #GP handler
        InterruptDescriptorTableRaw::hijack(sidt().base as _);

        cr0.set(Cr0::CR0_WRITE_PROTECT, true);
        unsafe { cr0_write(cr0) };

        // last but not least
        unsafe {
            wrmsr(IA32_LSTAR, hx_syscall_entry as _);
        }
    });
}

unsafe extern "C" {
    fn hx_syscall_entry();
}

global_asm!(include_str!("registers.inc"));
global_asm!(include_str!("context.asm"));
global_asm!(include_str!("idt.asm"));
global_asm!(include_str!("syscall.asm"));
