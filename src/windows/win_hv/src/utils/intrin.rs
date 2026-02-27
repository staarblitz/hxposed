use core::arch::{asm, global_asm, naked_asm};

#[unsafe(no_mangle)]
pub static mut ORIGINAL_GP_HANDLER: u64 = 0;

pub unsafe fn interlocked_increment(object: *mut u64) {
    unsafe { asm!("lock inc qword ptr [{0}]", in(reg) object) }
}

pub unsafe fn interlocked_decrement(object: *mut u64) {
    unsafe { asm!("lock dec qword ptr [{0}]", in(reg) object) }
}

pub fn rdmsr_failsafe(msr: u32) -> Option<u64> {
    let mut reg_value = 0u64;
    let mut failed = 0u64;
    unsafe {
        asm!("call rdmsr_failsafe_naked", in("rcx") msr, out("rax") reg_value, out("rdx") failed);
    }
    match failed {
        0 => Some(reg_value),
        _ => None,
    }
}

pub fn wrmsr_failsafe(msr: u32, value: u64) -> Option<()> {
    let mut failed = value;
    unsafe {
        asm!("call wrmsr_failsafe_naked", in("rcx") msr, in("rdx") value, out("rax") failed);
    }
    match failed {
        0 => Some(()),
        _ => None,
    }
}

// for interrupt handler, see hvcore/interrupt_handler.rs

global_asm!(include_str!("intrin.asm"));

unsafe extern "C" {
    pub(crate) fn hx_gp_handler();
    fn rdmsr_failsafe_naked(msr: u32) -> u64;
    fn wrmsr_failsafe_naked(msr: u32, value: u64) -> u8;
}