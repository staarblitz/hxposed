use core::arch::{asm, global_asm};
use x86::bits64::rflags::RFlags;
use x86::dtables::DescriptorTablePointer;
use x86::segmentation::SegmentSelector;

pub(crate) fn lsl(selector: SegmentSelector) -> u32 {
    let flags: u64;
    let mut limit: u64;
    unsafe {
        asm!(
        "lsl {}, {}",
        "pushfq",
        "pop {}",
        out(reg) limit,
        in(reg) u64::from(selector.bits()),
        lateout(reg) flags
        );
    };
    if RFlags::from_raw(flags).contains(RFlags::FLAGS_ZF) {
        limit as _
    } else {
        0
    }
}

pub(crate) fn lar(selector: SegmentSelector) -> u32 {
    let flags: u64;
    let mut access_rights: u64;
    unsafe {
        asm!(
        "lar {}, {}",
        "pushfq",
        "pop {}",
        out(reg) access_rights,
        in(reg) u64::from(selector.bits()),
        lateout(reg) flags
        );
    };
    if RFlags::from_raw(flags).contains(RFlags::FLAGS_ZF) {
        access_rights as _
    } else {
        0
    }
}

pub(crate) fn sidt() -> DescriptorTablePointer<u64> {
    let mut idtr = DescriptorTablePointer::<u64>::default();
    unsafe { x86::dtables::sidt(&mut idtr) };
    idtr
}

pub(crate) fn sgdt() -> DescriptorTablePointer<u64> {
    let mut gdtr = DescriptorTablePointer::<u64>::default();
    unsafe { x86::dtables::sgdt(&mut gdtr) };
    gdtr
}

pub(crate) fn tr() -> SegmentSelector {
    unsafe { x86::task::tr() }
}

pub(crate) fn ldtr() -> SegmentSelector {
    unsafe { x86::dtables::ldtr() }
}

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


global_asm!(include_str!("intrin.asm"));

#[allow(dead_code)]
unsafe extern "C" {
    fn hw_bp();
    fn rdmsr_failsafe_naked(msr: u32) -> u64;
    fn wrmsr_failsafe_naked(msr: u32, value: u64) -> u8;
}
