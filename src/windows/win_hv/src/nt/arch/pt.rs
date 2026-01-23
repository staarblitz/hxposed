use crate::nt::arch::{phys_to_virt};
use bitfield_struct::bitfield;
use hxposed_core::hxposed::requests::memory::{Pa, Pfn};

pub trait PagingEntry {
    type DownType;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType;
}

#[bitfield(u64)]
pub struct PageMapLevel5 {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub pwt: bool,
    pub pcd: bool,
    pub accessed: bool,
    pub ignored: bool,
    pub large: bool,
    pub global: bool,
    pub cow: bool,
    pub proto: bool,
    pub sf_write: bool,
    #[bits(40)]
    pub pfn: Pfn,
    #[bits(11)]
    pub reserved: u64,
    pub nx: bool,
}

impl PagingEntry for PageMapLevel5 {
    type DownType = PageMapLevel4;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType {
        let addr: u64 = self.pfn().into_phys().into();

        unsafe { &mut *(phys_to_virt(addr + (index as u64 * 8)) as *mut Self::DownType) }
    }
}

impl PageMapLevel5 {
    pub fn from_phys(addr: Pa, index: u16) -> &'static mut Self {
        let addr: u64 = addr.into();
        let addr = addr as *mut Self;
        unsafe {
            let ptr = phys_to_virt(addr.add(index as _).addr() as _) as *mut Self;
            ptr.as_mut().unwrap()
        }
    }
}

#[bitfield(u64)]
pub struct PageMapLevel4 {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub pwt: bool,
    pub pcd: bool,
    pub accessed: bool,
    pub ignored: bool,
    pub large: bool,
    #[bits(3)]
    pub ignored2: u64,
    pub global: bool,
    #[bits(40)]
    pub pfn: Pfn,
    #[bits(11)]
    pub reserved: u64,
    pub nx: bool,
}

impl PageMapLevel4 {
    pub fn from_phys(addr: Pa, index: u16) -> &'static mut Self {
        let addr: u64 = addr.into();
        let addr = addr as *mut Self;
        unsafe {
            let ptr = phys_to_virt(addr.add(index as _).addr() as _) as *mut Self;
            ptr.as_mut().unwrap()
        }
    }
}

impl PagingEntry for PageMapLevel4 {
    type DownType = PageDirectoryPointerEntry;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType {
        let addr: u64 = self.pfn().into_phys().into();

        unsafe { &mut *(phys_to_virt(addr + (index as u64 * 8)) as *mut Self::DownType) }
    }
}

#[bitfield(u64)]
pub struct PageDirectoryPointerEntry {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub pwt: bool,
    pub pcd: bool,
    pub accessed: bool,
    pub ignored: bool,
    pub large: bool,
    #[bits(3)]
    pub ignored2: u64,
    pub global: bool,
    #[bits(40)]
    pub pfn: Pfn,
    #[bits(11)]
    pub reserved: u64,
    pub nx: bool,
}

impl PagingEntry for PageDirectoryPointerEntry {
    type DownType = PageDirectoryEntry;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType {
        let addr: u64 = self.pfn().into_phys().into();

        unsafe { &mut *(phys_to_virt(addr + (index as u64 * 8)) as *mut Self::DownType) }
    }
}

#[bitfield(u64)]
pub struct PageDirectoryEntry {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub pwt: bool,
    pub pcd: bool,
    pub accessed: bool,
    pub ignored: bool,
    pub large: bool,
    #[bits(4)]
    pub ignored2: u64,
    #[bits(40)]
    pub pfn: Pfn,
    #[bits(11)]
    pub reserved: u64,
    pub nx: bool,
}

impl PagingEntry for PageDirectoryEntry {
    type DownType = PageTableEntry;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType {
        let addr: u64 = self.pfn().into_phys().into();

        unsafe { &mut *(phys_to_virt(addr + (index as u64 * 8)) as *mut Self::DownType) }
    }
}

#[bitfield(u64)]
pub struct PageTableEntry {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub pwt: bool,
    pub pcd: bool,
    pub accessed: bool,
    pub dirty: bool,
    pub pat: bool,
    pub global: bool,
    #[bits(3)]
    pub ignored2: u64,
    #[bits(40)]
    pub pfn: Pfn,
    #[bits(7)]
    pub reserved: u64,
    #[bits(4)]
    pub pk: u64,
    pub nx: bool,
}

impl PagingEntry for PageTableEntry {
    type DownType = u64;
    fn walk_down(&self, index: u16) -> &'static mut Self::DownType {
        let addr: u64 = self.pfn().into_phys().into();
        unsafe { &mut *(phys_to_virt(addr + (index as u64 * 8)) as *mut Self::DownType) }
    }
}
