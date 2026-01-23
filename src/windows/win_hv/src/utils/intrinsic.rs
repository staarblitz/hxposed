use core::arch::asm;
use crate::nt::arch::cr3::Cr3;

pub(crate) fn __readcr3() -> u64 {
    let mut cr3 = 0u64;
    unsafe {
        asm!("mov {0}, cr3", out(reg) cr3);
    }

    cr3
}

pub(crate) fn __writecr3(val: u64) {
    unsafe {
        asm!("mov cr3, {0}", in(reg) val);
    }
}