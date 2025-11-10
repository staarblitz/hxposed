#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;
extern crate wdk;
extern crate wdk_alloc;
extern crate wdk_sys;
use win::alloc::PoolAllocSized;

mod ops;
mod win;

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use bit_field::BitField;
use core::arch::asm;
use core::iter::once;
use core::ptr::null_mut;
use hv::SharedHostData;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::{HypervisorCall, HypervisorResult};
use hxposed_core::hxposed::error::{ErrorCode, ErrorSource};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::response::VmcallResponse;
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::status::HypervisorStatus;
use wdk::println;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{RtlInitUnicodeString, ZwOpenKey, ZwQueryValueKey};
use wdk_sys::{
    DRIVER_OBJECT, HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, NTSTATUS, OBJECT_ATTRIBUTES,
    PCUNICODE_STRING, POOL_FLAG_NON_PAGED, PVOID, STATUS_INSUFFICIENT_RESOURCES, STATUS_SUCCESS,
    UNICODE_STRING, ntddk::ExAllocatePool2,
};

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
extern "C" fn driver_entry(
    _driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    println!("Loading win_hv.sys");

    // Initialize the global allocator with allocated buffer.
    let ptr = unsafe {
        ExAllocatePool2(
            POOL_FLAG_NON_PAGED,
            hv::allocator::ALLOCATION_BYTES as _,
            0x2009,
        )
    };
    if ptr.is_null() {
        println!("Memory allocation failed");
        return STATUS_INSUFFICIENT_RESOURCES;
    }
    hv::allocator::init(ptr.cast::<u8>());

    // Register the platform specific API.
    hv::platform_ops::init(Box::new(ops::WindowsOps));

    // Virtualize the system. No `SharedHostData` is given, meaning that host's
    // IDT, GDT, TSS and page tables are all that of the system process (PID=4).
    // This makes the host debuggable with Windbg but also breakable from CPL0.

    let mut host_data = SharedHostData::default();
    host_data.vmcall_handler = Some(vmcall_handler);

    hv::virtualize_system(host_data);

    println!("Loaded win_hv.sys");
    STATUS_SUCCESS
}

fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    println!("Handling vmcall function: {:?}", info.func());
    match info.func() {
        ServiceFunction::Authorize => unsafe {
            // All other fields are ignored.

            let mut guid: u128 = guest.regs().r8 as _;
            guid.set_bits(64..127, guest.regs().r9 as _);

            let mut key_name = UNICODE_STRING::default();
            RtlInitUnicodeString(&mut key_name, as_utf16!("Status"));

            let mut object_attributes: OBJECT_ATTRIBUTES = Default::default();
            init_object_attributes!(
                &mut object_attributes,
                format!("\\Registry\\Machine\\Software\\HxPosed\\Plugins\\{}", guid),
                0,
                null_mut(),
                null_mut()
            );

            let mut key_handle = HANDLE::default();
            let status = ZwOpenKey(&mut key_handle, KEY_ALL_ACCESS, &mut object_attributes);
            if status != STATUS_SUCCESS {
                let err =
                    HypervisorResult::error(ErrorSource::Nt, ErrorCode::from_bits(status as _));
                guest.regs().rax = err.into_bits() as _;
                return;
            }

            let mut ret_len = 0;
            let _ = ZwQueryValueKey(
                key_handle,
                &mut key_name,
                KeyValueFullInformation,
                null_mut(),
                0,
                &mut ret_len,
            );

            let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(ret_len as _);

            let status = ZwQueryValueKey(
                key_handle,
                &mut key_name,
                KeyValueFullInformation,
                as_pvoid!(info),
                ret_len,
                &mut ret_len,
            );

            if status != STATUS_SUCCESS {
                let err =
                    HypervisorResult::error(ErrorSource::Nt, ErrorCode::from_bits(status as _));
                guest.regs().rax = err.into_bits() as _;
                return;
            }

            let data = get_data!(info, bool);
            if !*data {
                let err = HypervisorResult::error(ErrorSource::Hv, ErrorCode::NotAllowed);
                guest.regs().rax = err.into_bits() as _;
            }

            let ok = HypervisorResult::error(ErrorSource::Hv, ErrorCode::Ok);
            guest.regs().rax = ok.into_bits() as _;
        },
        ServiceFunction::GetState => {
            // All other fields of HxPosedCall are ignored.

            let response = StatusResponse {
                state: HypervisorStatus::SystemVirtualized,
                version: 1,
            }
            .into_raw();

            unsafe { asm!("int 0x3") }

            guest.regs().r8 = response.arg1;
            guest.regs().r9 = response.arg2;
            guest.regs().r10 = response.arg3;
            guest.regs().rsi = response.result.into_bits() as _;
        }
        ServiceFunction::Unknown => {}
    }
}
