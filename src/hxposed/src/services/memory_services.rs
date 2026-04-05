use crate::nt::arch::cr3::Cr3Context;
use crate::nt::arch::pt::{
    PageDirectoryEntry, PageDirectoryPointerEntry, PageMapLevel4, PageTableEntry, PagingEntry,
};
use crate::nt::mm::rmd::RawMemoryDescriptor;
use crate::nt::process::NtProcess;
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::requests::memory::*;
use hxposed_core::hxposed::responses::empty::EmptyResponse;
use hxposed_core::hxposed::responses::memory::*;
use hxposed_core::hxposed::responses::{HxResponse, SyscallResponse};

// I hate this so much
pub fn get_set_page_attribute(request: PageAttributeRequest) -> HxResponse {
    let cr = NtProcess::from_ptr(request.addr_space as _).get_user_directory_table_base();
    let _ctx = Cr3Context::begin(cr.into());

    let resp = match request.paging_type {
        PagingType::Unknown => return HxResponse::invalid_params(0),
        PagingType::Pml5(_) => unreachable!("No elegant way to choose between pml5 and 4 yet"),
        PagingType::Pml4(va) => unsafe {
            let field = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageMapLevel4::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => (*field).into_bits(),
            }
        },
        PagingType::Pdp(va) => unsafe {
            let pml4 = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match (*pml4).walk_down(va.get_pdp_index()) {
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageDirectoryPointerEntry::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => (*pml4).into_bits(),
            }
        },
        PagingType::Pd(va) => unsafe {
            let pml4 = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pdp = match (*pml4).walk_down(va.get_pdp_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match (*pdp).walk_down(va.get_pd_index()) {
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageDirectoryEntry::from_bits(request.type_bits);
                    0
                }
                PageAttributeOperation::Get => (*field).into_bits(),
            }
        },
        PagingType::Pt(va) => unsafe {
            let pdp = match PageMapLevel4::from_phys(cr, va.get_pml4_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pdp = match (*pdp).walk_down(va.get_pdp_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let pd = match (*pdp).walk_down(va.get_pd_index()) {
                Ok(field) if !(*field).present() => {
                    return HxResponse::not_found_what(NotFoundReason::Mdl);
                }
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };

            let field = match (*pd).walk_down(va.get_pt_index()) {
                Ok(field) => field,
                Err(_) => return HxResponse::not_found_what(NotFoundReason::Mdl),
            };
            match request.operation {
                PageAttributeOperation::Set => {
                    *field = PageTableEntry::from_bits(request.type_bits);
                    // TODO: invalidate page?
                    0
                }
                PageAttributeOperation::Get => (*field).into_bits(),
            }
        },
    };

    match resp {
        0 => EmptyResponse::default(),
        _ => PageAttributeResponse { type_bits: resp }.into_raw(),
    }
}

pub fn translate_address(request: TranslateAddressRequest) -> HxResponse {
    let current = NtProcess::current();
    let process = match current
        .get_object_tracker_unchecked()
        .get_open_process(request.addr_space)
    {
        Some(x) => x,
        None => return HxResponse::not_found_what(NotFoundReason::Process),
    };

    let _cr3 = Cr3Context::begin(process.get_user_directory_table_base().into());
    let virt = Va::from(request.virtual_addr);

    unsafe {
        let pml4 =
            match PageMapLevel4::from_phys(Pa::from(request.addr_space), virt.get_pml4_index()) {
                Ok(x) => x,
                Err(_) => {
                    return HxResponse::invalid_params(0);
                }
            };
        if !(*pml4).present() {
            return HxResponse::invalid_params(1);
        }
        let pdp = match (*pml4).walk_down(virt.get_pdp_index()) {
            Ok(x) => x,
            Err(_) => {
                return HxResponse::invalid_params(2);
            }
        };
        if !(*pdp).present() {
            return HxResponse::invalid_params(3);
        }
        let pd = match (*pdp).walk_down(virt.get_pd_index()) {
            Ok(x) => x,
            Err(_) => {
                return HxResponse::invalid_params(4);
            }
        };
        if !(*pd).present() {
            return HxResponse::invalid_params(5);
        }

        let pt = match (*pd).walk_down(virt.get_pt_index()) {
            Ok(x) => x,
            Err(_) => {
                return HxResponse::invalid_params(7);
            }
        };
        if !(*pt).present() {
            return HxResponse::invalid_params(7);
        }

        TranslateAddressResponse {
            physical_addr: <Pa as Into<u64>>::into((*pt).pfn().into_phys().into())
                + (virt.get_phys_offset() as u64),
        }
        .into_raw()
    }
}

pub fn map_va_to_pa(request: MapRmdRequest) -> HxResponse {
    // that's lame.
    // should I add a dispatcher?
    match request.operation {
        MapOperation::Map => {}
        MapOperation::Unmap => return unmap_va(request),
    }

    let process = NtProcess::from_ptr(request.addr_space as _);
    let tracker = process.get_object_tracker_unchecked();
    let rmd = match tracker.get_rmd(request.object) {
        None => return HxResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => x,
    };

    match rmd.map(process.clone(), request.map_addr) {
        Ok(_) => EmptyResponse::default(),
        Err(_) => HxResponse::not_allowed(NotAllowedReason::MappingsExist),
    }
}

pub fn unmap_va(request: MapRmdRequest) -> HxResponse {
    let process = NtProcess::from_ptr(request.addr_space as _);
    let tracker = process.get_object_tracker_unchecked();
    let rmd = match tracker.get_rmd(request.object) {
        None => return HxResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => x,
    };

    match rmd.find_map(&process, request.map_addr) {
        None => HxResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => match rmd.unmap(&x) {
            Ok(_) => EmptyResponse::default(),
            Err(_) => HxResponse::not_found_what(NotFoundReason::Mdl),
        },
    }
}

pub fn describe_memory(request: DescribeMemoryRequest) -> HxResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    let handle = tracker.add_rmd(RawMemoryDescriptor::describe_physical(
        Pa::from(request.pa),
        request.size,
    ));

    DescribeMemoryResponse { rmd: handle }.into_raw()
}

pub fn allocate_memory(request: AllocateMemoryRequest) -> HxResponse {
    let rmd = RawMemoryDescriptor::new_alloc(request.size, request.memory_type);

    let handle = NtProcess::current()
        .get_object_tracker_unchecked()
        .add_rmd(rmd);

    AllocateMemoryResponse { rmd: handle }.into_raw()
}

pub fn free_memory(request: FreeMemoryRequest) -> HxResponse {
    let process = NtProcess::current();
    let tracker = process.get_object_tracker_unchecked();
    match tracker.pop_rmd(request.obj) {
        None => HxResponse::not_found_what(NotFoundReason::Mdl),
        Some(x) => match x.free() {
            Ok(_) => EmptyResponse::default(),
            Err(_) => {
                // add it back
                tracker.add_rmd(x);
                HxResponse::not_allowed(NotAllowedReason::MappingsExist)
            }
        },
    }
}
