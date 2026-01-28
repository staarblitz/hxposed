use core::arch::asm;

pub unsafe fn interlocked_increment(object: *mut u64) {
    unsafe { asm!("lock inc qword ptr [{0}]", in(reg) object) }
}

pub unsafe fn interlocked_decrement(object: *mut u64) {
    unsafe { asm!("lock dec qword ptr [{0}]", in(reg) object) }
}

// for interrupt handler, see hvcore/interrupt_handler.rs

pub fn rdmsr_failsafe(msr: u32) -> Result<u64, ()> {
    let mut old_r15 = 0u64;

    unsafe { asm!("mov r15, {}", out(reg) old_r15) }

    let mut expected_r15 = 0x2009u64;

    let (high, low): (u32, u32);
    unsafe {
        asm!("rdmsr", out("eax") low, out("edx") high, in("ecx") msr, inout("r15") expected_r15);
    }

    let result = if expected_r15 == 0 {
        Err(())
    } else {
        Ok(((high as u64) << 32) | (low as u64))
    };

    unsafe { asm!("mov {}, r15", in(reg) old_r15) }

    result
}

pub fn wrmsr_failsafe(msr: u32, value: u64) -> Result<(), ()> {
    let mut old_r15 = 0u64;

    unsafe { asm!("mov r15, {}", out(reg) old_r15) }

    let mut expected_r15 = 0x2009u64;

    let low = value as u32;
    let high = (value >> 32) as u32;
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") low, in("edx") high, inout("r15") expected_r15);
    }

    // that means read triggered a #GP and it was catched
    let result = if expected_r15 == 0 { Err(()) } else { Ok(()) };

    unsafe { asm!("mov {}, r15", in(reg) old_r15) }

    result
}
