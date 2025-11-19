#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;

mod cback;
mod nt;
mod ops;
mod plugins;
mod services;
mod win;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use crate::cback::registry_callback;
use crate::nt::get_nt_info;
use crate::plugins::plugin::Plugin;
use crate::plugins::{PluginTable, load_plugins};
use crate::win::Utf8ToUnicodeString;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::ops::{BitAnd, DerefMut};
use core::panic::Location;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use hv::SharedHostData;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::func::ServiceFunction::Authorize;
use hxposed_core::hxposed::requests::VmcallRequest;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;
use hxposed_core::plugins::plugin_perms::PluginPermissions;
use wdk::println;

use crate::services::authorize_plugin;
use wdk_alloc::WdkAllocator;
use wdk_sys::ntddk::{CmRegisterCallback, KeBugCheckEx};
use wdk_sys::{
    DRIVER_OBJECT, LARGE_INTEGER, NTSTATUS, PCUNICODE_STRING, POOL_FLAG_NON_PAGED, PVOID,
    STATUS_INSUFFICIENT_RESOURCES, STATUS_SUCCESS, STATUS_TOO_LATE, ntddk::ExAllocatePool2,
};

static CM_COOKIE: AtomicU64 = AtomicU64::new(0);
static PLUGINS: AtomicPtr<PluginTable> = AtomicPtr::new(null_mut());

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
extern "C" fn driver_entry(
    _driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    get_nt_info();

    if nt::NT_BUILD.load(Ordering::Relaxed) != 26200 {
        println!("Unsupported version");
        return STATUS_TOO_LATE;
    }

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
    // This makes the host debuggable with WinDbg but also breakable from CPL0.

    let mut host_data = SharedHostData::default();
    host_data.vmcall_handler = Some(vmcall_handler);

    hv::virtualize_system(host_data);

    load_plugins();

    println!("Loaded win_hv.sys");

    let mut cookie = LARGE_INTEGER::default();
    let status = unsafe {
        CmRegisterCallback(
            Some(registry_callback),
            PVOID::default(), /* What lol */
            &mut cookie,
        )
    };

    unsafe {
        CM_COOKIE.store(cookie.QuadPart as _, Ordering::Relaxed);
    }

    if status != STATUS_SUCCESS {
        println!("Error registering registry callbacks");
    }

    STATUS_SUCCESS
}

///
/// # Called when a CPUID with RCX = 2009 is executed.
///
/// # Arguments
/// guest - The trait of guest. Intel or AMD.
///
/// info - Information about the call. See [HypervisorCall].
///
/// # Return
/// There is no return value of this function, however, the return value of the vmcall will be in RSI.
/// Which you *may* want to utilize. See documentation on GitHub page for more information about trap ABI.
///
/// # Warning
///
/// ## We are in context of the thread that made the vmcall.
/// Functions like "IoGetCurrentProcess" returns the process that made the vmcall, not the system process.
/// (that is a good thing)
///
/// ## IRQL is above sane.
/// IRQL is 255, all interrupts are disabled. Using Zw* and other functions that ask for PASSIVE_LEVEL will only result in tears.
///
/// ## This is a VMEXIT handler
/// Don't you dare to "take your time". This interrupts the whole CPU and making the kernel scheduler forget its purpose.
///
fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    println!("Handling vmcall function: {:?}", info.func());

    if info.func() == ServiceFunction::GetState {
        write_response(
            guest,
            StatusResponse {
                state: HypervisorStatus::SystemVirtualized,
                version: 1,
            }
            .into_raw(),
        );
    }

    let args = get_args(guest);

    let plugin = match Plugin::current() {
        None => {
            if info.func() == Authorize {
                authorize_plugin(guest, AuthorizationRequest::from_raw(info, args));
                return;
            }
            write_response(
                guest,
                HypervisorResponse::not_allowed(
                    NotAllowedReason::PluginNotLoaded,
                    PluginPermissions::empty(),
                ),
            );
            return;
        }
        Some(x) => x,
    };

    match info.func() {
        ServiceFunction::OpenProcess | ServiceFunction::CloseProcess => {
            services::handle_process_services(guest, info, args, plugin)
        }
        ServiceFunction::Unknown => {}
        _ => {}
    }
}

pub(crate) fn get_args(guest: &mut dyn Guest) -> (u64, u64, u64) {
    (guest.regs().r8, guest.regs().r9, guest.regs().r10)
}

pub(crate) fn write_response(guest: &mut dyn Guest, response: HypervisorResponse) {
    guest.regs().r8 = response.arg1;
    guest.regs().r9 = response.arg2;
    guest.regs().r10 = response.arg3;
    guest.regs().rsi = response.result.into_bits() as _;
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic occurred: {:?}", _info);

    let param1 = _info
        .message()
        .as_str()
        .unwrap_or("Could not unwrap message");
    let param2 = _info.location().unwrap_or(Location::caller());

    // First parameter is the message.
    // Second parameter is the column and line encoded. First 32 bits (LSB) is column, next 32 bits are the line.
    // Third parameter is the file location.
    // Fourth parameter is reserved.

    unsafe {
        KeBugCheckEx(
            0x2009,
            param1.as_ptr() as _,
            (param2.column() as u64 | ((param2.line() as u64) << 31)) as _,
            param2.file().as_ptr() as _,
            0,
        )
    };
}
