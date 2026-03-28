use crate::size_assert;
use crate::utils::logger::HxLogger;
use alloc::boxed::Box;
use core::ops::Add;
use x86::msr::{IA32_FS_BASE, rdmsr, IA32_GS_BASE};
use hxposed_core::hxposed::responses::HxResponse;

#[repr(C, align(16))]
#[derive(Default, Clone, Debug)]
pub struct Registers {
    pub rax: u64, // 0
    pub rbx: u64, // 8
    pub rcx: u64, // 16
    pub rdx: u64, // 24
    pub rsi: u64, // 32
    pub rdi: u64, // 40
    pub r8: u64,  // 48
    pub r9: u64,  // 56
    pub r10: u64, // 64
    pub r11: u64, // 72
    pub r12: u64, // 80
    pub r13: u64, // 88
    pub r14: u64, // 96
    pub r15: u64, // 104
    pub rbp: u64, // 112

    pub rflags: u64, // 120

    pub xmm0: u128, // 128
    pub xmm1: u128, // 144
    pub xmm2: u128, // 160
    pub xmm3: u128, // 176
    pub xmm4: u128, // 192
    pub xmm5: u128, // 208

    pub rsp: u64, // 224
    pub rip: u64, // 232
}
size_assert!(Registers, 240);

unsafe extern "C" {
    pub unsafe fn hx_capture_context(regs: &mut Registers);
}

#[repr(C)]
pub struct HxFs {
    pub logger: Box<HxLogger>,
    pub registers: Box<Registers>,
    pub stack: u64,
}

impl HxFs {
    pub fn new() -> Self {
       Self {
            logger: Box::new(HxLogger::new()),
            registers: unsafe {Box::new_zeroed().assume_init()},
            stack: Box::into_raw(Box::<[u8;4096]>::new_zeroed()) as _,
        }
    }

    pub fn write_response(&mut self, response: HxResponse) {
        self.registers.rsi = response.result.into_bits();
        self.registers.r8 = response.arg1;
        self.registers.r9 = response.arg2;
        self.registers.r10 = response.arg3;
    }
    
    pub fn get_current() -> &'static mut Self {
        unsafe { &mut *(rdmsr(IA32_GS_BASE).add(0x78) as *mut Self) }
    }
}
