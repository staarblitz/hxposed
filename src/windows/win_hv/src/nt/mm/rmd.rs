use crate::nt::arch::pt::{PageDirectoryPointerEntry, PageMapLevel4, PageMapLevel5, PagingEntry};
use crate::nt::arch::{phys_to_virt, virt_to_phys};
use crate::nt::process::NtProcess;
use crate::utils::transaction::Transaction;
use core::arch::asm;
use hxposed_core::hxposed::requests::memory::{Pa, Pfn, Va};
use wdk_sys::ntddk::{ExAllocatePool2, ExFreePool, ExFreePool2};
use wdk_sys::{POOL_FLAG_NON_PAGED, PVOID};
use crate::utils::intrinsic::*;

pub struct RawMemoryDescriptor {}

impl RawMemoryDescriptor {
    pub fn map(process: &NtProcess, phys: Pfn, virt: Va) -> Result<(), ()> {
        let base = process.get_dtb();
        let mut tx = Transaction::new();

        // before anything, we have to switch our CR3 to base. so our virtual address resolution via MmGetVirtualForPhysical won't get us garbage.
        let cr3 = __readcr3();
        __writecr3(base.into());

/*        let pml5 = PageMapLevel5::from_phys(phys, virt.get_pml5_index());
        if !pml5.present() {
            let addr =
                Pa::from(virt_to_phys(Self::alloc_page_aligned().get_page_addr())).into_pfn();
            tx.enlist(move || Self::free_page_aligned(addr));
            pml5.set_pfn(addr)
        }*/

        let pml4 = PageMapLevel4::from_phys(base ,virt.get_pml4_index());
        if !pml4.present() {
            let addr =
                Pa::from(virt_to_phys(Self::alloc_page_aligned().into())).into_pfn();
            tx.enlist(move || Self::free_page_aligned(addr));
            pml4.set_pfn(addr)
        }

        let pdp = pml4.walk_down(virt.get_pdp_index());
        if !pdp.present() {
            let addr =
                Pa::from(virt_to_phys(Self::alloc_page_aligned().into())).into_pfn();
            tx.enlist(move || Self::free_page_aligned(addr));
            pdp.set_pfn(addr)
        }

        let pd = pdp.walk_down(virt.get_pd_index());
        if !pd.present() {
            let addr =
                Pa::from(virt_to_phys(Self::alloc_page_aligned().into())).into_pfn();
            tx.enlist(move || Self::free_page_aligned(addr));
            pd.set_pfn(addr)
        }

        let pt = pd.walk_down(virt.get_pt_index());
        if !pt.present() {
            pt.set_pfn(phys)
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

        // reset back
        __writecr3(cr3);

        Ok(())
    }

    pub fn unmap(virt: Va) -> Result<(), ()> {
        let base = Pa::from(virt_to_phys(virt.get_page_addr()));

        let pml5 = PageMapLevel5::from_phys(base, virt.get_pml5_index());
        if !pml5.present() {
            return Err(());
        }

        let pml4 = pml5.walk_down(virt.get_pml4_index());
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

    fn alloc_page_aligned() -> Va {
        unsafe { Va::from(ExAllocatePool2(POOL_FLAG_NON_PAGED, 4096, 0x2026) as u64) }
    }

    fn free_page_aligned(addr: Pfn) {
        unsafe {
            ExFreePool(Va::from(phys_to_virt(Pa::from_pfn(addr).into())).get_addr() as _);
        }
    }
}
