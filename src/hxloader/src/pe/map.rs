#![allow(unsafe_op_in_unsafe_fn)]

use crate::nt::*;
use crate::pe::HxLoaderParameterBlock;
use crate::utils::get_cstr_len;
use core::ptr::{copy_nonoverlapping, null_mut};
// https://github.com/memN0ps/redlotus-rs/blob/master/bootkit/src/mapper/mod.rs
// with some modifications

pub unsafe fn manually_map(
    nt_base: *const u8,
    new_base: *mut u8,
    old_base: *const u8,
) -> *const u8 {
    log::info!("HxPosed new base: {:x}", new_base as usize);
    log::info!("HxPosed old base: {:x}", old_base as usize);

    log::info!("Copying headers...");
    copy_headers(old_base, new_base);

    log::info!("Copying sections...");
    copy_sections(old_base, new_base);

    log::info!("Rebasing image...");
    rebase_image(old_base, new_base);

    log::info!("Fixing imports...");
    resolve_imports(new_base, nt_base);

    log::info!("Manually mapped!");
    let headers = &*get_nt_headers(new_base);

    log::info!("HxPosed.sys base: {:x}", new_base as usize);
    log::info!(
        "HxPosed.sys size: {:x}",
        headers.OptionalHeader.SizeOfImage as usize
    );

    log::info!("Adding HxLoaderParameters...");
    match get_section_by_name(headers, ".hxprm") {
        None => {
            panic!("HxPosed section not found!");
        }
        Some(params) => {
            let cfg = ((&*params).VirtualAddress as usize + new_base as usize)
                as *mut HxLoaderParameterBlock;

            log::info!(".hxprm found at: {:x}", cfg as usize);

            let cfg = &mut *cfg;
            cfg.base_address = new_base as _;
            cfg.pe_size = headers.OptionalHeader.SizeOfImage as _;
            cfg.booted_from_hxloader = true;
        }
    };

    // we cannot return the entry point in nt headers because it points to Gs/Fx and not the actual one.
    // it causes stack failure, as you would guess.
    //new_base.byte_offset(headers.OptionalHeader.AddressOfEntryPoint as _)

    get_function_by_name(new_base, "DriverEntry".as_ref())
}

pub unsafe fn get_section_by_name(
    headers: &IMAGE_NT_HEADERS64,
    name: &str,
) -> Option<PIMAGE_SECTION_HEADER> {
    let mut current_section_ptr = (&headers.OptionalHeader as *const _ as usize
        + headers.FileHeader.SizeOfOptionalHeader as usize)
        as *mut IMAGE_SECTION_HEADER;

    let count = headers.FileHeader.NumberOfSections;

    for _ in 0..count {
        let section = &mut *current_section_ptr;

        let name_bytes = &section.Name;
        let len = name_bytes.iter().position(|&c| c == 0).unwrap_or(8);

        if let Ok(name_str) = str::from_utf8(&name_bytes[..len]) {
            if name_str == name {
                return Some(section);
            }
        }

        current_section_ptr = current_section_ptr.add(1);
    }

    None
}

pub unsafe fn resolve_imports(module_base: *const u8, ntoskrnl_base: *const u8) -> Option<bool> {
    let nt_headers = get_nt_headers(module_base);

    // Get a pointer to the first _IMAGE_IMPORT_DESCRIPTOR
    let mut import_directory = (module_base as usize
        + (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize]
            .VirtualAddress as usize) as PIMAGE_IMPORT_DESCRIPTOR;

    if import_directory.is_null() {
        return Some(false);
    }

    while (*import_directory).Name != 0x0 {
        // Get a pointer to the Original Thunk or First Thunk via OriginalFirstThunk or FirstThunk
        let mut original_thunk = if (module_base as usize
            + (*import_directory).Anonymous.OriginalFirstThunk as usize)
            != 0
        {
            let orig_thunk = (module_base as usize
                + (*import_directory).Anonymous.OriginalFirstThunk as usize)
                as PIMAGE_THUNK_DATA64;
            orig_thunk
        } else {
            let thunk = (module_base as usize + (*import_directory).FirstThunk as usize)
                as PIMAGE_THUNK_DATA64;
            thunk
        };

        if original_thunk.is_null() {
            return Some(false);
        }

        let mut thunk =
            (module_base as usize + (*import_directory).FirstThunk as usize) as PIMAGE_THUNK_DATA64;

        if thunk.is_null() {
            return Some(false);
        }

        while (*original_thunk).u1.Function != 0 {
            // Get a pointer to _IMAGE_IMPORT_BY_NAME
            let thunk_data = (module_base as usize + (*original_thunk).u1.AddressOfData as usize)
                as PIMAGE_IMPORT_BY_NAME;

            // Get a pointer to the function name in the IMAGE_IMPORT_BY_NAME
            let fn_name = (*thunk_data).Name.as_ptr();
            let fn_len: usize = get_cstr_len(fn_name);
            let fn_slice = core::slice::from_raw_parts(fn_name, fn_len);
            //log::info!("fn_name: {:?}", String::from_utf8_lossy(fn_slice));

            (*thunk).u1.Function = get_function_by_name(ntoskrnl_base, fn_slice) as _;

            // Increment and get a pointer to the next Thunk and Original Thunk
            thunk = thunk.add(1);
            original_thunk = original_thunk.add(1);
        }

        // Increment and get a pointer to the next _IMAGE_IMPORT_DESCRIPTOR
        import_directory =
            (import_directory as usize + size_of::<IMAGE_IMPORT_DESCRIPTOR>() as usize) as _;
    }

    Some(true)
}

unsafe fn get_function_by_name(module_base: *const u8, name_slice: &[u8]) -> *const u8 {
    let nt_headers = &*get_nt_headers(module_base);

    let export = nt_headers.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT as usize]
        .VirtualAddress;
    if export == 0 {
        log::error!("Export virtual address empty!");
        return null_mut();
    }

    let exports = &mut *(module_base.byte_offset(export as _) as PIMAGE_EXPORT_DIRECTORY);
    let number_of_names = exports.NumberOfNames;
    let names = module_base.byte_offset(exports.AddressOfNames as _) as *mut u32;

    for i in 0..number_of_names {
        let function = module_base.byte_offset(*names.add(i as _) as _);
        let function_name = core::slice::from_raw_parts(function, get_cstr_len(function));

        if function_name == name_slice {
            let func_addr = module_base.byte_offset(exports.AddressOfFunctions as _) as *mut u32;
            let ordinal = module_base.byte_offset(exports.AddressOfNameOrdinals as _) as *mut u16;
            return module_base.byte_offset(*func_addr.add(*ordinal.add(i as _) as _) as _);
        }
    }

    null_mut()
}

unsafe fn rva_to_file_offset(old_base: *const u8, rva: u32) -> Option<usize> {
    let nt = &*get_nt_headers(old_base);

    let sections = ((&nt.OptionalHeader as *const _ as usize)
        + nt.FileHeader.SizeOfOptionalHeader as usize) as PIMAGE_SECTION_HEADER;

    for i in 0..nt.FileHeader.NumberOfSections {
        let s = &*sections.add(i as usize);

        let start = s.VirtualAddress;
        let end = start + s.SizeOfRawData.max(s.Misc.VirtualSize);

        if rva >= start && rva < end {
            return Some((rva - start + s.PointerToRawData) as usize);
        }
    }

    // rwa points to headers
    if rva < nt.OptionalHeader.SizeOfHeaders {
        return Some(rva as usize);
    }

    None
}

unsafe fn rebase_image(old_base: *const u8, new_base: *mut u8) {
    let nt_headers = &*get_nt_headers(old_base);
    let relocations =
        &nt_headers.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_BASERELOC as usize];

    if relocations.Size == 0 {
        return; // HELLA YEA!
    }

    let mut file_offset =
        rva_to_file_offset(old_base, relocations.VirtualAddress).expect("invalid reloc directory");

    let reloc_end = file_offset + relocations.Size as usize;

    let delta = new_base.addr() as isize - nt_headers.OptionalHeader.ImageBase as isize;

    while file_offset < reloc_end {
        let block = &*(old_base.add(file_offset) as *const IMAGE_BASE_RELOCATION);

        if block.SizeOfBlock < size_of::<IMAGE_BASE_RELOCATION>() as u32 {
            break;
        }

        let entry_count =
            (block.SizeOfBlock as usize - size_of::<IMAGE_BASE_RELOCATION>()) / size_of::<u16>();

        let entries =
            (block as *const _ as *const u8).add(size_of::<IMAGE_BASE_RELOCATION>()) as *const u16;

        let page_base = new_base.add(block.VirtualAddress as usize);

        for i in 0..entry_count {
            let entry = *entries.add(i);
            let ty = (entry >> 12) as u32;
            let off = entry & 0x0FFF;

            let target = page_base.add(off as usize);

            match ty {
                IMAGE_REL_BASED_DIR64 => {
                    *(target as *mut u64) += delta as u64;
                }
                IMAGE_REL_BASED_HIGHLOW => {
                    *(target as *mut u32) += delta as u32;
                }
                _ => {}
            }
        }

        file_offset += block.SizeOfBlock as usize;
    }
}

unsafe fn copy_sections(old_base: *const u8, new_base: *mut u8) {
    let nt_headers = get_nt_headers(old_base);
    let section_header = (&(*nt_headers).OptionalHeader as *const _ as usize
        + (*nt_headers).FileHeader.SizeOfOptionalHeader as usize)
        as PIMAGE_SECTION_HEADER;

    for i in 0..(*nt_headers).FileHeader.NumberOfSections {
        let section_header_i = &*(section_header.add(i as usize));
        if section_header_i.SizeOfRawData == 0 {
            // BSS fucking bullshit whatever
            core::ptr::write_bytes(
                new_base.add(section_header_i.VirtualAddress as usize),
                0,
                section_header_i.Misc.VirtualSize as usize,
            );
            continue;
        }
        let destination = new_base.add(section_header_i.VirtualAddress as usize);
        let source = (old_base as usize + section_header_i.PointerToRawData as usize) as *const u8;
        let size = section_header_i
            .SizeOfRawData
            .min(section_header_i.Misc.VirtualSize) as usize;

        copy_nonoverlapping(source, destination, size);
    }
}

unsafe fn copy_headers(old_base: *const u8, new_base: *mut u8) {
    let nt_headers = &*get_nt_headers(old_base);

    copy_nonoverlapping(
        old_base,
        new_base,
        nt_headers.OptionalHeader.SizeOfHeaders as _,
    );
}

pub unsafe fn get_nt_headers(image_base: *const u8) -> PIMAGE_NT_HEADERS64 {
    let dos_header = image_base as PIMAGE_DOS_HEADER;

    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        log::error!("IMAGE_DOS_SIGNATURE does not match");
        panic!();
    }

    let nt_headers = image_base.byte_offset((*dos_header).e_lfanew as isize) as PIMAGE_NT_HEADERS64;
    if (*nt_headers).Signature != IMAGE_NT_SIGNATURE {
        log::error!("IMAGE_NT_SIGNATURE does not match");
        panic!();
    }

    nt_headers
}
