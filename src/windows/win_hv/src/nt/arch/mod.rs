use crate::win::{MmGetPhysicalAddress, MmGetVirtualForPhysical};
use bit_field::BitField;

pub(crate) mod cr3;
pub(crate) mod pt;

pub fn virt_to_phys(virt: u64) -> u64 {
    unsafe { MmGetPhysicalAddress(virt as _) }
}

pub fn phys_to_virt(phys: u64) -> u64 {
    unsafe { MmGetVirtualForPhysical(phys) as _ }
}
