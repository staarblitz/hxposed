#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;
extern crate wdk;
extern crate wdk_alloc;
extern crate wdk_sys;
use win::alloc::PoolAllocSized;

mod cback;
mod ops;
mod win;

use crate::cback::registry_callback;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use alloc::boxed::Box;
use alloc::format;
use core::ops::BitAnd;
use core::ptr::null_mut;
use core::sync::atomic::AtomicU64;
use hv::SharedHostData;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::{HypervisorCall, HypervisorResult};
use hxposed_core::hxposed::error::{ErrorCode, ErrorSource};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::responses::VmcallResponse;
use hxposed_core::hxposed::responses::auth::AuthorizationResponse;
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::status::HypervisorStatus;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use uuid::Uuid;
use wdk::{dbg_break, println};
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{
    CmRegisterCallback, RtlInitUnicodeString, ZwCreateKey, ZwOpenKey, ZwQueryValueKey,
};
use wdk_sys::{DRIVER_OBJECT, HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, LARGE_INTEGER, NTSTATUS, OBJ_KERNEL_HANDLE, OBJECT_ATTRIBUTES, PCUNICODE_STRING, POOL_FLAG_NON_PAGED, PVOID, REG_OPENED_EXISTING_KEY, REG_OPTION_NON_VOLATILE, STATUS_INSUFFICIENT_RESOURCES, STATUS_SUCCESS, UNICODE_STRING, ntddk::ExAllocatePool2, OBJ_CASE_INSENSITIVE};

static mut CM_COOKIE: AtomicU64 = AtomicU64::new(0);

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

    // let mut cookie = LARGE_INTEGER::default();
    // let status = unsafe {
    //     CmRegisterCallback(
    //         Some(registry_callback),
    //         PVOID::default(), /* What lol */
    //         &mut cookie,
    //     )
    // };
    // if status != STATUS_SUCCESS {
    //     println!("Error registering registry callbacks");
    // }

    STATUS_SUCCESS
}

fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    println!("Handling vmcall function: {:?}", info.func());
    match info.func() {
        ServiceFunction::Authorize => unsafe {
            // All other fields are ignored.

            dbg_break();

            let req = AuthorizationRequest {
                uuid: Uuid::from_u64_pair(guest.regs().r8, guest.regs().r9),
                permissions: PluginPermissions::from_bits(guest.regs().r10).unwrap(),
            };

            let mut key_name = "Permissions".to_unicode_string();

            let mut object_attributes: OBJECT_ATTRIBUTES = Default::default();
            InitializeObjectAttributes(
                &mut object_attributes,
                format!(
                    "\\Registry\\Machine\\SOFTWARE\\HxPosed\\Plugins\\{}",
                    req.uuid
                )
                .as_str()
                .to_unicode_string()
                .as_mut(),
                OBJ_KERNEL_HANDLE | OBJ_CASE_INSENSITIVE,
                Default::default(),
                Default::default(),
            );

            let mut key_handle = HANDLE::default();
            let status = ZwCreateKey(
                &mut key_handle,
                KEY_ALL_ACCESS,
                &mut object_attributes,
                0,
                Default::default(),
                REG_OPTION_NON_VOLATILE,
                REG_OPENED_EXISTING_KEY as _,
            );
            if status != STATUS_SUCCESS {
                let err =
                    HypervisorResult::error(ErrorSource::Nt, ErrorCode::from_bits(status as _));
                guest.regs().rax = err.into_bits() as _;
                return;
            }

            let mut ret_len = 0;
            let _ = ZwQueryValueKey(
                key_handle,
                key_name.as_mut(),
                KeyValueFullInformation,
                Default::default(),
                0,
                &mut ret_len,
            );

            let mut info = KEY_VALUE_FULL_INFORMATION::alloc_sized(ret_len as _);

            let status = ZwQueryValueKey(
                key_handle,
                key_name.as_mut(),
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

            let data =  *get_data!(info, PluginPermissions);

            // And the masks to find out allowed permissions.
            let permissions = data.bitand(req.permissions);

            let resp = AuthorizationResponse { permissions }.into_raw();

            guest.regs().r8 = resp.arg1;
            guest.regs().r9 = resp.arg2;
            guest.regs().r10 = resp.arg3;
            guest.regs().rsi = resp.result.into_bits() as _;
        },
        ServiceFunction::GetState => {
            // All other fields of HxPosedCall are ignored.

            let resp = StatusResponse {
                state: HypervisorStatus::SystemVirtualized,
                version: 1,
            }
            .into_raw();

            guest.regs().r8 = resp.arg1;
            guest.regs().r9 = resp.arg2;
            guest.regs().r10 = resp.arg3;
            guest.regs().rsi = resp.result.into_bits() as _;
        }
        ServiceFunction::Unknown => {}
        _ => {}
    }
}
