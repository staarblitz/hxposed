use bit_field::BitField;
use core::arch::asm;
use hxposed_core::hxposed::requests::memory::Pfn;

pub struct Cr3(u64);

impl Cr3 {
    pub fn get_base(self) -> Pfn {
        Pfn::from_bits(self.0.get_bits(12..63))
    }

    pub(crate) fn read_raw() -> u64 {
        let mut cr3 = 0u64;
        unsafe {
            asm!("mov {0}, cr3", out(reg) cr3);
        }

        cr3
    }

    pub(crate) fn write_raw(val: u64) {
        unsafe {
            asm!("mov cr3, {0}", in(reg) val);
        }
    }
}

pub struct Cr3Context {
    old: u64,
}

impl Drop for Cr3Context {
    fn drop(&mut self) {
        Cr3::write_raw(self.old);
    }
}

impl Cr3Context {
    pub fn begin(cr3_base: u64) -> Self {
        let me = Cr3Context {
            old: Cr3::read_raw(),
        };
        Cr3::write_raw(cr3_base);
        me
    }
}
