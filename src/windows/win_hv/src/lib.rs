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
mod plugins;
mod registry;
mod win;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use crate::plugins::plugin::Plugin;
use crate::registry::registry_timer;
use crate::win::{InitializeObjectAttributes, Utf8ToUnicodeString};
use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::ops::{BitAnd, DerefMut};
use core::sync::atomic::{AtomicU64, Ordering};
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
use spin::mutex::SpinMutex;
use uuid::Uuid;
use wdk::{dbg_break, println};
use wdk_alloc::WdkAllocator;
use wdk_sys::_KEY_VALUE_INFORMATION_CLASS::KeyValueFullInformation;
use wdk_sys::ntddk::{CmRegisterCallback, KeBugCheck, PsCreateSystemThread, ZwCreateKey, ZwQueryValueKey};
use wdk_sys::{DRIVER_OBJECT, HANDLE, KEY_ALL_ACCESS, KEY_VALUE_FULL_INFORMATION, NTSTATUS, OBJ_CASE_INSENSITIVE, OBJ_KERNEL_HANDLE, OBJECT_ATTRIBUTES, PCLIENT_ID, PCUNICODE_STRING, POBJECT_ATTRIBUTES, POOL_FLAG_NON_PAGED, PVOID, REG_OPENED_EXISTING_KEY, REG_OPTION_NON_VOLATILE, STATUS_INSUFFICIENT_RESOURCES, STATUS_SUCCESS, THREAD_ALL_ACCESS, ntddk::ExAllocatePool2, LARGE_INTEGER};
use crate::cback::registry_callback;

static CM_COOKIE: AtomicU64 = AtomicU64::new(0);
static PLUGINS_DB: SpinMutex<Vec<Plugin>> = SpinMutex::new(Vec::new());

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

    unsafe {
        let mut handle = HANDLE::default();
        let _ = PsCreateSystemThread(
            &mut handle,
            THREAD_ALL_ACCESS,
            POBJECT_ATTRIBUTES::default(),
            HANDLE::default(),
            PCLIENT_ID::default(),
            Some(registry_timer),
            PVOID::default(),
        );
    }

    // let mut cookie = LARGE_INTEGER::default();
    // let status = unsafe {
    //     CmRegisterCallback(
    //         Some(registry_callback),
    //         PVOID::default(), /* What lol */
    //         &mut cookie,
    //     )
    // };
    //
    // unsafe {
    //     CM_COOKIE.store(cookie.QuadPart as _, Ordering::Relaxed);
    // }
    //
    // if status != STATUS_SUCCESS {
    //     println!("Error registering registry callbacks");
    // }

    STATUS_SUCCESS
}

fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    println!("Handling vmcall function: {:?}", info.func());
    match info.func() {
        ServiceFunction::Authorize => {
            // All other fields are ignored.

            let req = AuthorizationRequest {
                uuid: Uuid::from_u64_pair(guest.regs().r8, guest.regs().r9),
                permissions: PluginPermissions::from_bits(guest.regs().r10).unwrap(),
            };

            let mut plugins = PLUGINS_DB.lock();
            let mut plugin: Option<Plugin> = None;

            // its a shame .iter() is not available in no_std
            for item in plugins.deref_mut() {
                if item.uuid == req.uuid {
                    plugin = Some(item.clone());
                }
            }

            if plugin.is_none() {
                let error = HypervisorResult::error(ErrorSource::Hx, ErrorCode::NotAllowed);
                guest.regs().rsi = error.into_bits() as _;
                return;
            }

            // And the masks to find out allowed permissions.
            let permissions = plugin.unwrap().permissions.bitand(req.permissions);

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

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic occurred: {:?}", _info);
    unsafe { KeBugCheck(0x2009) };
}
