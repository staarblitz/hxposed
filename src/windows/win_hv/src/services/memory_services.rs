use crate::nt::arch::cr3::Cr3Context;
use crate::nt::arch::pt::{
    PageDirectoryEntry, PageDirectoryPointerEntry, PageMapLevel4, PageMapLevel5, PageTableEntry,
    PagingEntry,
};
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
    let cr = NtProcess::from_ptr(request.addr_space as _).get_user_directory_table_base();
    let _ctx = Cr3Context::begin(cr.into());

    let resp = match request.paging_type {
        PagingType::Pml5(_) => unreachable!("No elegant way to choose between pml5 and 4 yet"),
        PagingType::Pml4(va) => {
            let field = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageMapLevel4::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => field.into_bits(),
            }
        }
        PagingType::Pdp(va) => {
            let pml4 = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match pml4.walk_down(va.get_pdp_index()) {
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageDirectoryPointerEntry::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => field.into_bits(),
            }
        }
        PagingType::Pd(va) => {
            let pml4 = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pdp = match pml4.walk_down(va.get_pdp_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match pdp.walk_down(va.get_pd_index()) {
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageDirectoryEntry::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => field.into_bits(),
            }
        }
        PagingType::Pt(va) => {
            let pdp = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pdp = match pdp.walk_down(va.get_pdp_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pd = match pdp.walk_down(va.get_pd_index()) {
                Ok(field) if !field.present() => {
                    return HypervisorResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match pd.walk_down(va.get_pt_index()) {
                Ok(field) => field,
                Err(_) => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageTableEntry::from_bits(request.type_bits);
                    // TODO: invalidate page?
                    0
                }
                PageAttributeOperation::Get => field.into_bits(),
            }
        }
    };

    match resp {
        0 => EmptyResponse::with_service(ServiceFunction::GetSetPageAttribute),
        _ => PageAttributeResponse { type_bits: resp }.into_raw(),
    }
}

pub fn map_va_to_pa(request: MapVaToPaRequest) -> HypervisorResponse {
    // that's lame.
    // should I add a dispatcher?
    match request.operation {
        MapOperation::Map => {}
        MapOperation::Unmap => return unmap_va(request),
    }

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

pub fn unmap_va(request: MapVaToPaRequest) -> HypervisorResponse {
    let process = NtProcess::from_ptr(request.addr_space as _);
    let tracker = process.get_object_tracker_unchecked();
    let rmd = match tracker.get_rmd(request.object) {
        None => return HypervisorResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => x,
    };

    match rmd.find_map(&process, request.map_addr) {
        None => HypervisorResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => match rmd.unmap(&x) {
            Ok(_) => EmptyResponse::with_service(ServiceFunction::MapMemory),
            Err(_) => HypervisorResponse::not_found(),
        },
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
