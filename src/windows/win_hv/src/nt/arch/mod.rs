use bit_field::BitField;
use wdk_sys::PHYSICAL_ADDRESS;
use wdk_sys::ntddk::{MmGetPhysicalAddress, MmGetVirtualForPhysical};

pub(crate) mod cr3;
pub(crate) mod pt;


pub fn virt_to_phys(virt: u64) -> u64 {
    unsafe { MmGetPhysicalAddress(virt as _).QuadPart as _ }
}

pub fn phys_to_virt(phys: u64) -> u64 {
    unsafe {
        MmGetVirtualForPhysical(PHYSICAL_ADDRESS {
            QuadPart: phys as _,
        }) as _
    }
}
