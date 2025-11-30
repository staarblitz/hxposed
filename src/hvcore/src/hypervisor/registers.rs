use core::arch::global_asm;
use core::ops::BitOr;

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub rip: u64,
    pub xmm0: Xmm,
    pub xmm1: Xmm,
    pub xmm2: Xmm,
    pub xmm3: Xmm,
    pub xmm4: Xmm,
    pub xmm5: Xmm,
}
const _: () = assert!(core::mem::size_of::<Registers>() == 0xf0);

impl Registers {
    #[inline(always)]
    pub(crate) fn capture_current() -> Self {
        let mut registers = Registers::default();
        unsafe { capture_registers(&mut registers) };
        registers
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Xmm {
    pub low: u64,
    pub hight: u64,
}

impl Into<u128> for Xmm {
    fn into(self) -> u128 {
        (self.low as u128).bitor(((self.hight as u128) << 63))
    }
}

unsafe extern "C" {
    /// Captures current register values.
    fn capture_registers(registers: &mut Registers);
}
global_asm!(include_str!("capture_registers.inc"));
global_asm!(include_str!("capture_registers.S"));
