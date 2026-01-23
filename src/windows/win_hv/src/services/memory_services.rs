use wdk_sys::ntddk::{ExAllocatePool2, ExFreePool, MmAllocateContiguousMemory, MmFreeContiguousMemory};
use wdk_sys::{PHYSICAL_ADDRESS, POOL_FLAG_NON_PAGED};
use crate::nt::arch::pt::{PageMapLevel5, PagingEntry};
use crate::nt::arch::{phys_to_virt, virt_to_phys};
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::memory::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use crate::nt::mm::rmd::RawMemoryDescriptor;

pub fn get_set_page_attribute(request: PageAttributeRequest) -> HypervisorResponse {
    let cr = NtProcess::from_ptr(request.addr_space as _).get_dtb();
    match request.operation {
        PageAttributeOperation::Set => {
            match request.paging_type {
                PagingType::Pml5(index) => {
                    let field = PageMapLevel5::from_phys(cr, index);
                    match request.attributes {
                        PageAttributes::Present(x) => field.set_present(x),
                        PageAttributes::Writable(x) => field.set_write(x),
                        PageAttributes::UserAccessible(x) => field.set_user(x),
                        PageAttributes::WriteThrough(x) => field.set_pwt(x),
                        PageAttributes::NoCache(x) => field.set_pcd(x),
                        PageAttributes::Accessed(x) => field.set_accessed(x),
                        PageAttributes::Large(x) => field.set_large(x),
                        PageAttributes::Global(x) => field.set_global(x),
                        PageAttributes::Pfn(x) => field.set_pfn(Pfn::from_bits(x)),
                        PageAttributes::ExecuteDisable(x) => field.set_nx(x),
                        PageAttributes::CopyOnWrite(x) => field.set_cow(x),
                        PageAttributes::SoftwareWrite(x) => field.set_sf_write(x),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pml4(index, index2) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let field = pml5.walk_down(index2);
                    match request.attributes {
                        PageAttributes::Present(x) => field.set_present(x),
                        PageAttributes::Writable(x) => field.set_write(x),
                        PageAttributes::UserAccessible(x) => field.set_user(x),
                        PageAttributes::WriteThrough(x) => field.set_pwt(x),
                        PageAttributes::NoCache(x) => field.set_pcd(x),
                        PageAttributes::Accessed(x) => field.set_accessed(x),
                        PageAttributes::Large(x) => field.set_large(x),
                        PageAttributes::Global(x) => field.set_global(x),
                        PageAttributes::Pfn(x) => field.set_pfn(Pfn::from_bits(x)),
                        PageAttributes::ExecuteDisable(x) => field.set_nx(x),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pdp(index, index2, index3) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let field = pml4.walk_down(index3);
                    match request.attributes {
                        PageAttributes::Present(x) => field.set_present(x),
                        PageAttributes::Writable(x) => field.set_write(x),
                        PageAttributes::UserAccessible(x) => field.set_user(x),
                        PageAttributes::WriteThrough(x) => field.set_pwt(x),
                        PageAttributes::NoCache(x) => field.set_pcd(x),
                        PageAttributes::Accessed(x) => field.set_accessed(x),
                        PageAttributes::Large(x) => field.set_large(x),
                        PageAttributes::Global(x) => field.set_global(x),
                        PageAttributes::Pfn(x) => field.set_pfn(Pfn::from_bits(x)),
                        PageAttributes::ExecuteDisable(x) => field.set_nx(x),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pd(index, index2, index3, index4) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let pdp = pml4.walk_down(index3);
                    let field = pdp.walk_down(index4);
                    match request.attributes {
                        PageAttributes::Present(x) => field.set_present(x),
                        PageAttributes::Writable(x) => field.set_write(x),
                        PageAttributes::UserAccessible(x) => field.set_user(x),
                        PageAttributes::WriteThrough(x) => field.set_pwt(x),
                        PageAttributes::NoCache(x) => field.set_pcd(x),
                        PageAttributes::Accessed(x) => field.set_accessed(x),
                        PageAttributes::Large(x) => field.set_large(x),
                        PageAttributes::Pfn(x) => field.set_pfn(Pfn::from_bits(x)),
                        PageAttributes::ExecuteDisable(x) => field.set_nx(x),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pt(index, index2, index3, index4, index5) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let pdp = pml4.walk_down(index3);
                    let pd = pdp.walk_down(index4);
                    let field = pd.walk_down(index5);
                    match request.attributes {
                        PageAttributes::Present(x) => field.set_present(x),
                        PageAttributes::Writable(x) => field.set_write(x),
                        PageAttributes::UserAccessible(x) => field.set_user(x),
                        PageAttributes::WriteThrough(x) => field.set_pwt(x),
                        PageAttributes::NoCache(x) => field.set_pcd(x),
                        PageAttributes::Accessed(x) => field.set_accessed(x),
                        PageAttributes::Global(x) => field.set_global(x),
                        PageAttributes::Pfn(x) => field.set_pfn(Pfn::from_bits(x)),
                        PageAttributes::ExecuteDisable(x) => field.set_nx(x),
                        PageAttributes::Dirty(x) => field.set_dirty(x),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
            }

            EmptyResponse::with_service(ServiceFunction::GetSetPageAttribute)
        }
        PageAttributeOperation::Get => {
            let response = match request.paging_type {
                PagingType::Pml5(index) => {
                    let field = PageMapLevel5::from_phys(cr, index);
                    match request.attributes {
                        PageAttributes::Present(_) => PageAttributes::Present(field.present()),
                        PageAttributes::Writable(_) => PageAttributes::Writable(field.write()),
                        PageAttributes::UserAccessible(_) => {
                            PageAttributes::UserAccessible(field.user())
                        }
                        PageAttributes::WriteThrough(_) => {
                            PageAttributes::WriteThrough(field.pwt())
                        }
                        PageAttributes::NoCache(_) => PageAttributes::NoCache(field.pcd()),
                        PageAttributes::Accessed(_) => PageAttributes::Accessed(field.accessed()),
                        PageAttributes::Large(_) => PageAttributes::Large(field.large()),
                        PageAttributes::Global(_) => PageAttributes::Global(field.global()),
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        PageAttributes::CopyOnWrite(_) => PageAttributes::CopyOnWrite(field.cow()),
                        PageAttributes::SoftwareWrite(_) => {
                            PageAttributes::SoftwareWrite(field.sf_write())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pml4(index, index2) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let field = pml5.walk_down(index2);
                    match request.attributes {
                        PageAttributes::Present(_) => PageAttributes::Present(field.present()),
                        PageAttributes::Writable(_) => PageAttributes::Writable(field.write()),
                        PageAttributes::UserAccessible(_) => {
                            PageAttributes::UserAccessible(field.user())
                        }
                        PageAttributes::WriteThrough(_) => {
                            PageAttributes::WriteThrough(field.pwt())
                        }
                        PageAttributes::NoCache(_) => PageAttributes::NoCache(field.pcd()),
                        PageAttributes::Accessed(_) => PageAttributes::Accessed(field.accessed()),
                        PageAttributes::Large(_) => PageAttributes::Large(field.large()),
                        PageAttributes::Global(_) => PageAttributes::Global(field.global()),
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pdp(index, index2, index3) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let field = pml4.walk_down(index3);
                    match request.attributes {
                        PageAttributes::Present(_) => PageAttributes::Present(field.present()),
                        PageAttributes::Writable(_) => PageAttributes::Writable(field.write()),
                        PageAttributes::UserAccessible(_) => {
                            PageAttributes::UserAccessible(field.user())
                        }
                        PageAttributes::WriteThrough(_) => {
                            PageAttributes::WriteThrough(field.pwt())
                        }
                        PageAttributes::NoCache(_) => PageAttributes::NoCache(field.pcd()),
                        PageAttributes::Accessed(_) => PageAttributes::Accessed(field.accessed()),
                        PageAttributes::Large(_) => PageAttributes::Large(field.large()),
                        PageAttributes::Global(_) => PageAttributes::Global(field.global()),
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pd(index, index2, index3, index4) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let pdp = pml4.walk_down(index3);
                    let field = pdp.walk_down(index4);
                    match request.attributes {
                        PageAttributes::Present(_) => PageAttributes::Present(field.present()),
                        PageAttributes::Writable(_) => PageAttributes::Writable(field.write()),
                        PageAttributes::UserAccessible(_) => {
                            PageAttributes::UserAccessible(field.user())
                        }
                        PageAttributes::WriteThrough(_) => {
                            PageAttributes::WriteThrough(field.pwt())
                        }
                        PageAttributes::NoCache(_) => PageAttributes::NoCache(field.pcd()),
                        PageAttributes::Accessed(_) => PageAttributes::Accessed(field.accessed()),
                        PageAttributes::Large(_) => PageAttributes::Large(field.large()),
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pt(index, index2, index3, index4, index5) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    let pml4 = pml5.walk_down(index2);
                    let pdp = pml4.walk_down(index3);
                    let pd = pdp.walk_down(index4);
                    let field = pd.walk_down(index5);
                    match request.attributes {
                        PageAttributes::Present(_) => PageAttributes::Present(field.present()),
                        PageAttributes::Writable(_) => PageAttributes::Writable(field.write()),
                        PageAttributes::UserAccessible(_) => {
                            PageAttributes::UserAccessible(field.user())
                        }
                        PageAttributes::WriteThrough(_) => {
                            PageAttributes::WriteThrough(field.pwt())
                        }
                        PageAttributes::NoCache(_) => PageAttributes::NoCache(field.pcd()),
                        PageAttributes::Accessed(_) => PageAttributes::Accessed(field.accessed()),
                        PageAttributes::Global(_) => PageAttributes::Global(field.global()),
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        PageAttributes::Dirty(_) => PageAttributes::Dirty(field.dirty()),
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
            };

            PageAttributeResponse { result: response }.into_raw()
        }
    }
}

pub fn map_va_to_pa(request: MapVaToPaRequest) -> HypervisorResponse {
    if !request.virt.is_multiple_of(4096) {
        return HypervisorResponse::invalid_param();
    }

    if !request.phys.is_multiple_of(4096) {
        return HypervisorResponse::invalid_param();
    }

    let va = Va::from(request.virt);
    let pa = Pa::from(request.phys);
    let cr = NtProcess::from_ptr(request.addr_space as _);

    match RawMemoryDescriptor::map(&cr, pa.into_pfn(), va) {
        Ok(_) => EmptyResponse::with_service(ServiceFunction::MapVaToPa),
        Err(_) => HypervisorResponse::not_found()
    }
}

pub fn allocate_memory(request: AllocateMemoryRequest) -> HypervisorResponse {
    // we need to be in 4096 byte bound. or we die

    let size = ((request.size + 4095) / 4096) * 4096;

    let ptr = match request.memory_type {
        MemoryType::NonPagedPool => unsafe {
            ExAllocatePool2(POOL_FLAG_NON_PAGED, size as _, 0x2009)
        }
        MemoryType::ContiguousPhysical => unsafe{
            MmAllocateContiguousMemory(size as _, PHYSICAL_ADDRESS {QuadPart: u64::MAX as _})
        }
    };

    AllocateMemoryResponse {
        system_pa: virt_to_phys(ptr as _)
    }.into_raw()
}

pub fn free_memory(request: FreeMemoryRequest) -> HypervisorResponse {
    match request.memory_type {
        MemoryType::NonPagedPool => unsafe{ ExFreePool(request.system_va as _)},
        MemoryType::ContiguousPhysical => unsafe{MmFreeContiguousMemory(request.system_va as _)},
    }

    EmptyResponse::with_service(ServiceFunction::FreeMemory)
}