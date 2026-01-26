use alloc::vec::Vec;
use crate::nt::arch::cr3::Cr3Context;
use crate::nt::arch::pt::{PageMapLevel4, PagingEntry};
use crate::nt::arch::virt_to_phys;
use crate::nt::process::NtProcess;
use crate::utils::transaction::Transaction;
use core::arch::asm;
use core::hash::Hash;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use spin::mutex::SpinMutex;
use hxposed_core::hxposed::requests::memory::{MemoryType, Pa, Va};
use wdk_sys::ntddk::{
    ExAllocatePool2, ExFreePool, MmAllocateContiguousMemory, MmFreeContiguousMemory,
};
use wdk_sys::{PEPROCESS, PHYSICAL_ADDRESS, POOL_FLAG_NON_PAGED};

#[derive(Debug)]
pub struct RawMemoryDescriptor {
    pub pa: Pa,
    pub system_va: Va,
    pub size: u32,
    pub memory_type: MemoryType,
    pub mapped_addrs: SpinMutex<Vec<MapDetails>>
}

#[derive(Debug)]
struct MapDetails {
    pub mapped_addr: u64,
    pub mapped_process: NtProcess
}

impl Hash for RawMemoryDescriptor {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.pa.into());
    }
}

impl RawMemoryDescriptor {
    pub fn new_alloc(size: u32, memory_type: MemoryType) -> Self {
        // we need to be in 4096 byte bound. or we die

        let size = ((size + 4095) / 4096) * 4096;

        let ptr = match memory_type {
            MemoryType::NonPagedPool => unsafe {
                ExAllocatePool2(POOL_FLAG_NON_PAGED, size as _, 0x2009)
            },
            MemoryType::ContiguousPhysical => unsafe {
                MmAllocateContiguousMemory(
                    size as _,
                    PHYSICAL_ADDRESS {
                        QuadPart: u64::MAX as _,
                    },
                )
            },
        };

        Self {
            system_va: Va::from(ptr as u64),
            pa: Pa::from(virt_to_phys(ptr as _)),
            mapped_addrs: SpinMutex::new(Vec::with_capacity(48)),
            memory_type,
            size,
        }
    }

    fn new_for_paging() -> Self {
        Self::new_alloc(4096, MemoryType::NonPagedPool)
    }

    fn from_raw(ptr: u64, size: u32, memory_type: MemoryType) -> Self {
        Self {
            system_va: Va::from(ptr),
            pa: Pa::from(virt_to_phys(ptr)),
            mapped_addrs: SpinMutex::new(Vec::with_capacity(48)),
            memory_type,
            size,
        }
    }

    pub fn teardown(&self) {
        let lock = self.mapped_addrs.lock();
        lock.iter().for_each(|details| {
            match self.unmap(&details.mapped_process, details.mapped_addr) {
                Ok(_) => {}
                Err(_) => {
                    log::error!("Failed to unmap {:?} during teardown. System unstable!", details);
                }
            }
        });

        // this should not fail since we already freed all mappings
        self.free().unwrap();
    }

    pub fn free(&self) -> Result<(), ()> {
        if self.mapped_addrs.lock().len() != 0 {
            return Err(());
        }

        match self.memory_type {
            MemoryType::NonPagedPool => unsafe { ExFreePool(self.system_va.get_addr() as _) },
            MemoryType::ContiguousPhysical => unsafe {
                MmFreeContiguousMemory(self.system_va.get_addr() as _)
            },
        }

        Ok(())
    }

    pub fn map(&self, process: NtProcess, map_addr: u64) -> Result<(), ()> {
        let virt = Va::from(map_addr);
        // kpti is on. so we have to use user directory table base.
        let base = process.get_user_directory_table_base();
        let mut tx = Transaction::new();

        // before anything, we have to switch our CR3 to base. so our virtual address resolution via MmGetVirtualForPhysical won't get us garbage.
        let _ctx = Cr3Context::begin(base.into());

        /*        let pml5 = PageMapLevel5::from_phys(phys, virt.get_pml5_index());
        if !pml5.present() {
            let addr =
                Pa::from(virt_to_phys(Self::alloc_page_aligned().get_page_addr())).into_pfn();
            tx.enlist(move || Self::free_page_aligned(addr));
            pml5.set_pfn(addr)
        }*/

        let pml4 = PageMapLevel4::from_phys(base, virt.get_pml4_index());
        pml4.make_user_accessible();
        if !pml4.present() {
            let addr = Self::new_for_paging();
            pml4.set_pfn(addr.pa.into_pfn());
            tx.enlist(move || addr.free().unwrap());
        }

        let pdp = pml4.walk_down(virt.get_pdp_index());
        pdp.make_user_accessible();
        if !pdp.present() {
            let addr = Self::new_for_paging();
            pdp.set_pfn(addr.pa.into_pfn());
            tx.enlist(move || addr.free().unwrap());
        }

        let pd = pdp.walk_down(virt.get_pd_index());
        pd.make_user_accessible();
        if !pd.present() {
            let addr = Self::new_for_paging();
            pd.set_pfn(addr.pa.into_pfn());
            tx.enlist(move || addr.free().unwrap());
        }

        let pt = pd.walk_down(virt.get_pt_index());
        pt.make_user_accessible();
        if !pt.present() {
            pt.set_pfn(self.pa.into_pfn())
        } else {
            // something occupies this address
            drop(tx);
            return Err(());
        }

        tx.commit();

        pt.set_present(true);
        pd.set_present(true);
        pdp.set_present(true);
        pml4.set_present(true);
        /*pml5.set_present(true);*/

        unsafe {
            asm!("invlpg ({})", in(reg) (virt.get_addr() as usize), options(att_syntax, nostack, preserves_flags));
        }

        self.mapped_addrs.lock().push(MapDetails {
            mapped_process: process,
            mapped_addr: map_addr,
        });

        Ok(())
    }

    pub fn unmap(&self, process: &NtProcess, mapped_addr: u64) -> Result<(), ()> {
        let mut lock = self.mapped_addrs.lock();

        // this is not failsafe. hmmm
        let map_details = match lock.iter().position(|details| {
            details.mapped_addr == mapped_addr && details.mapped_process.nt_process == process.nt_process
        }) {
            None => return Err(()),
            Some(x) => lock.remove(x),
        };

        let virt = Va::from(map_details.mapped_addr);
        let _ctx = Cr3Context::begin(map_details.mapped_process.get_user_directory_table_base().into());

        let base = Pa::from(virt_to_phys(virt.get_page_addr()));

        /*let pml5 = PageMapLevel5::from_phys(base, virt.get_pml5_index());
        if !pml5.present() {
            return Err(());
        }*/

        let pml4 = PageMapLevel4::from_phys(base, virt.get_pml4_index());
        if !pml4.present() {
            return Err(());
        }

        let pdp = pml4.walk_down(virt.get_pdp_index());
        if !pdp.present() {
            return Err(());
        }

        let pd = pdp.walk_down(virt.get_pdp_index());
        if !pd.present() {
            return Err(());
        }

        let pt = pd.walk_down(virt.get_pt_index());
        if pt.present() {
            pt.set_present(false);
            Ok(())
        } else {
            Err(())
        }
    }
}
