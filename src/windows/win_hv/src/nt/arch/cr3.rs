use core::ops::BitAnd;
use bit_field::BitField;
use hxposed_core::hxposed::requests::memory::Pfn;

pub struct Cr3(u64);

impl Cr3 {
    pub fn get_base(self) -> Pfn {
        Pfn::from_bits(self.0.get_bits(12..63))
    }
}