
use crate::nt::arch::pt::{PageMapLevel5, PagingEntry};
use crate::nt::arch::virt_to_phys;
use crate::nt::mm::rmd::RawMemoryDescriptor;
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::memory::*;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};

// I hate this so much
pub fn get_set_page_attribute(request: PageAttributeRequest) -> HypervisorResponse {
    let cr = NtProcess::from_ptr(request.addr_space as _).get_directory_table_base();

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
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
                    let field = pml5.walk_down(index2);
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
                PagingType::Pdp(index, index2, index3) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
                    let field = pml4.walk_down(index3);
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
                PagingType::Pd(index, index2, index3, index4) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pdp = pml4.walk_down(index3);
                    if !pdp.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
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
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pdp = pml4.walk_down(index3);
                    if !pdp.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
                    let pd = pdp.walk_down(index4);
                    if !pd.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
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
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

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
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pdp(index, index2, index3) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
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
                        PageAttributes::Pfn(_) => PageAttributes::Pfn(field.pfn().into_bits()),
                        PageAttributes::ExecuteDisable(_) => {
                            PageAttributes::ExecuteDisable(field.nx())
                        }
                        _ => return HypervisorResponse::invalid_param(),
                    }
                }
                PagingType::Pd(index, index2, index3, index4) => {
                    let pml5 = PageMapLevel5::from_phys(cr, index);
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pdp = pml4.walk_down(index3);
                    if !pdp.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
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
                    if !pml5.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pml4 = pml5.walk_down(index2);
                    if !pml4.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }

                    let pdp = pml4.walk_down(index3);
                    if !pdp.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
                    let pd = pdp.walk_down(index4);
                    if !pd.present() {
                        return HypervisorResponse::not_allowed(NotAllowedReason::PageNotPresent);
                    }
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
    let process = NtProcess::from_ptr(request.addr_space as _);
    let tracker = process.get_object_tracker_unchecked();
    let rmd = match tracker.get_rmd(request.object) {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => x,
    };

    match rmd.map(process.clone(), request.map_addr) {
        Ok(_) => EmptyResponse::with_service(ServiceFunction::MapVaToPa),
        Err(_) => HypervisorResponse::not_found(),
    }
}

pub fn allocate_memory(request: AllocateMemoryRequest) -> HypervisorResponse {
    let rmd = RawMemoryDescriptor::new_alloc(request.size, request.memory_type);
    let ptr = rmd.system_va.get_addr();
    NtProcess::current()
        .get_object_tracker_unchecked()
        .add_rmd(rmd);

    AllocateMemoryResponse {
        system_pa: virt_to_phys(ptr as _),
    }
    .into_raw()
}

pub fn free_memory(request: FreeMemoryRequest) -> HypervisorResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    match tracker.pop_rmd(request.obj) {
        None => HypervisorResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => match x.free() {
            Ok(_) => EmptyResponse::with_service(ServiceFunction::FreeMemory),
            Err(_) => {
                // add it back
                tracker.add_rmd(x);
                HypervisorResponse::not_allowed(NotAllowedReason::MappingsExist)
            }
        },
    }
}