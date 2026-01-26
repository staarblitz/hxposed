use core::arch::asm;

pub unsafe fn interlocked_increment(object: *mut u64) {
    unsafe {
        asm!("lock inc qword ptr [{0}]", in(reg) object)
    }
}

pub unsafe fn interlocked_decrement(object: *mut u64) {
    unsafe {
        asm!("lock dec qword ptr [{0}]", in(reg) object)
    }
}