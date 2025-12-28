use core::arch::asm;
use crate::nt::winload::Winload;
use crate::nt::*;
use crate::pe::map::manually_map;
use crate::pe::{BASIC_CALL, BASIC_CALL_PROLOGUE};
use crate::*;
use core::mem;
use core::ptr::null_mut;
use core::sync::atomic::Ordering;
use uefi::{CStr16, Char16, Status};

pub extern "C" fn img_arch_start_boot_application(
    app_entry: *mut u8,
    image_base: *mut u8,
    image_size: u32,
    boot_option: u8,
    return_arguments: *mut u8,
) -> Status {
    let mut detour = IMG_ARCH_START_BOOT_APPLICATION_DETOUR.lock();

    detour.revert();

    log::trace!("ImgArchStartBootApplication");

    Winload::new(image_base as _, image_size as _).patch();

    let func: ImgArchStartBootApplicationType = unsafe { mem::transmute(detour.original_address) };

    drop(detour);
    unsafe {
        func(
            app_entry,
            image_base,
            image_size,
            boot_option,
            return_arguments,
        )
    }
}

pub extern "C" fn bl_img_allocate_image_buffer(
    image_buffer: *mut *mut u8,
    image_size: u64,
    memory_type: u32,
    preferred_attributes: u32,
    preferred_alignment: u32,
    flags: u32,
) -> Status {
    log::trace!("BlImgAllocateImageBuffer");

    let mut detour = BL_IMG_ALLOCATE_IMAGE_BUFFER_DETOUR.lock();

    detour.revert();

    let func: BlImgAllocateImageBufferType = unsafe { mem::transmute(detour.original_address) };

    let status = unsafe {
        func(
            image_buffer,
            image_size,
            memory_type,
            preferred_attributes,
            preferred_alignment,
            flags,
        )
    };

    log::info!("Result: {:x}", status.0);

    if status == Status::SUCCESS && memory_type == 0xE0000012 {
        log::info!("Allocating for HxPosed...");
        let mut alloc = 0u64;
        // allocate again, but for ourselves
        let status = unsafe {
            func(
                (&mut alloc) as *mut _ as _,
                EFI_DRIVER_SIZE.load(Ordering::Relaxed),
                memory_type,
                0x424000, // RWX
                preferred_alignment,
                0,
            )
        };
        log::info!("Allocation result: {:x}", status.0);
        log::info!("Allocation: {:x}", alloc);
        NT_DRIVER_ALLOCATION.store(alloc as _, Ordering::Relaxed);

        drop(detour);

        log::info!("End of BlImgAllocateImageBuffer phase.");
        return status;
    }

    log::info!("Allocation was none of our interest. Rehook...");

    detour.detour();
    drop(detour);
    log::info!("Return status: {:x}", status.0);
    status
}

pub extern "C" fn osl_fwp_kernel_setup_phase1(loader: *mut _LOADER_PARAMETER_BLOCK) -> Status {
    log::trace!("OslFwpKernelSetupPhase1");
    log::info!("_LOADER_PARAMETER_BLOCK: {:x}", loader.addr());
    let mut detour = OSL_FWP_KERNEL_SETUP_PHASE1_DETOUR.lock();

    detour.revert();
    let func: OslFwpKernelSetupPhase1Type = unsafe { mem::transmute(detour.original_address) };
    drop(detour);

    let block = unsafe { &mut *loader };
    let first = &block.LoadOrderListHead as *const _LIST_ENTRY;
    let mut current = block.LoadOrderListHead.Flink as *const _LIST_ENTRY;

    let mut ntoskrnl_entry = null_mut::<_KLDR_DATA_TABLE_ENTRY>();
    let mut acpi_entry = null_mut::<_KLDR_DATA_TABLE_ENTRY>();

    log::info!("Searching for ntoskrnl.exe and acpi.sys");

    while first.addr() != current.addr() {
        let entry = unsafe { &mut *(current as *mut _KLDR_DATA_TABLE_ENTRY) };

        let name_slice = unsafe {
            core::slice::from_raw_parts::<Char16>(
                entry.BaseDllName.Buffer as *mut _,
                entry.BaseDllName.Length as usize,
            )
        };
        let name = CStr16::from_char16_until_nul(&name_slice).unwrap();

        if name == cstr16!("ntoskrnl.exe") {
            ntoskrnl_entry = entry;
        } else if name == cstr16!("ACPI.sys") {
            acpi_entry = entry
        }

        unsafe {
            current = (*current).Flink;
        }
    }

    log::info!("ntoskrnl entry: {:x}", ntoskrnl_entry.addr());
    log::info!("acpi entry: {:x}", acpi_entry.addr());

    let ntoskrnl_entry = unsafe { &mut *ntoskrnl_entry };
    let acpi_entry = unsafe { &mut *acpi_entry };
    log::info!("acpi.sys base: {:x}", acpi_entry.DllBase.addr());
    log::info!("acpi.sys size: {:x}", acpi_entry.SizeOfImage);

    log::info!("ntoskrnl base: {:x}", ntoskrnl_entry.DllBase.addr());
    log::info!("ntoskrnl size: {:x}", ntoskrnl_entry.SizeOfImage);

    let mapped_entry = unsafe {
        manually_map(
            ntoskrnl_entry.DllBase as _,
            NT_DRIVER_ALLOCATION.load(Ordering::Relaxed),
            EFI_DRIVER_ALLOCATION.load(Ordering::Relaxed),
        )
    };

    log::info!(
        "Mapped HxPosed.sys. New entry point: {:x}",
        mapped_entry.addr()
    );

    log::info!("Placing a call acpi.sys entry point...");

    unsafe {
        core::ptr::copy_nonoverlapping(
            BASIC_CALL.as_ptr(),
            acpi_entry.EntryPoint as *mut u8,
            BASIC_CALL.len(),
        );
        let addr = acpi_entry.EntryPoint.byte_offset(2) as *mut u64;
        addr.write(mapped_entry as u64);

        // the stack is unaligned at GsDriverEntry. we fixed it, and now we should restore it.

        core::ptr::copy_nonoverlapping(
            BASIC_CALL_PROLOGUE.as_ptr(),
            acpi_entry.EntryPoint.byte_offset((BASIC_CALL.len() + 5) as _) as _,
            BASIC_CALL_PROLOGUE.len(),
        );
    }

    log::info!("All good. We wish you a nice exposing experience...");

    unsafe { func(loader) }
}
