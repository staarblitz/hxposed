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
use crate::plugins::{load_plugins, PluginTable};
use crate::win::Utf8ToUnicodeString;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use core::ops::{BitAnd, DerefMut};
use core::panic::Location;
use core::ptr::{null_mut, slice_from_raw_parts_mut};
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use hv::hypervisor::host::Guest;
use hv::SharedHostData;
use hxposed_core::hxposed::call::HypervisorCall;
use hxposed_core::hxposed::error::NotAllowedReason;
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::func::ServiceFunction::Authorize;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;
use hxposed_core::services::async_service::AsyncInfo;
use wdk::println;

use crate::nt::worker::async_worker_thread;
use crate::services::authorize_plugin;
use wdk_alloc::WdkAllocator;
use wdk_sys::ntddk::{
    CmRegisterCallback, KeBugCheckEx, ProbeForRead, PsCreateSystemThread, ZwClose,
};
use wdk_sys::{
    ntddk::ExAllocatePool2, DRIVER_OBJECT, HANDLE, LARGE_INTEGER, NTSTATUS, PCUNICODE_STRING,
    PDRIVER_OBJECT, POOL_FLAG_NON_PAGED, PVOID, STATUS_INSUFFICIENT_RESOURCES, STATUS_SUCCESS,
    STATUS_TOO_LATE, THREAD_ALL_ACCESS,
};

static CM_COOKIE: AtomicU64 = AtomicU64::new(0);
static PLUGINS: AtomicPtr<PluginTable> = AtomicPtr::new(null_mut());

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
extern "C" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
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

    hv::platform_ops::init(Box::new(ops::WindowsOps));

    // TODO: use custom gdt and so on for more security?
    let mut host_data = SharedHostData::default();
    host_data.vmcall_handler = Some(vmcall_handler);

    hv::virtualize_system(host_data);

    load_plugins();

    driver.DriverUnload = Some(driver_unload);

    println!("Loaded win_hv.sys");

    let mut cookie = LARGE_INTEGER::default();
    match unsafe {
        CmRegisterCallback(
            Some(registry_callback),
            PVOID::default(), /* What lol */
            &mut cookie,
        )
    } {
        STATUS_SUCCESS => unsafe { CM_COOKIE.store(cookie.QuadPart as _, Ordering::Relaxed) },
        err => {
            panic!("Error registering registry callbacks: {:x}", err);
        }
    }

    let mut handle = HANDLE::default();
    match unsafe {
        PsCreateSystemThread(
            &mut handle,
            THREAD_ALL_ACCESS,
            Default::default(),
            Default::default(),
            Default::default(),
            Some(async_worker_thread),
            Default::default(),
        )
    } {
        STATUS_SUCCESS => unsafe {
            ZwClose(handle);
        },
        err => {
            panic!("Error creating system thread: {:x}", err);
        }
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
        return;
    }

    let mut request = HypervisorRequest {
        call: info,
        arg1: guest.regs().r8,
        arg2: guest.regs().r9,
        arg3: guest.regs().r10,
        async_info: Default::default(),
        extended_arg1: guest.regs().xmm0.into(),
        extended_arg2: guest.regs().xmm1.into(),
        extended_arg3: guest.regs().xmm2.into(),
        extended_arg4: guest.regs().xmm3.into(),
    };

    if info.is_async() {
        match microseh::try_seh(|| unsafe {
            ProbeForRead(guest.regs().r12 as _, 16, 1);
        }) {
            Ok(_) => {
                request.async_info = AsyncInfo {
                    handle: guest.regs().r11,
                    result_values: AtomicPtr::new(unsafe {
                        &mut *(slice_from_raw_parts_mut(guest.regs().r12 as *mut u64, 4)
                            as *mut [u64; 4])
                    }), // rsi, r8, r9, r10. total 4
                };
            }
            Err(_) => {}
        }
    }

    let plugin = match Plugin::current() {
        None => {
            if info.func() == Authorize {
                authorize_plugin(guest, AuthorizationRequest::from_raw(&request));
                return;
            }
            write_response(
                guest,
                HypervisorResponse::not_allowed(NotAllowedReason::PluginNotLoaded),
            );
            return;
        }
        Some(x) => x,
    };

    match info.func() {
        ServiceFunction::OpenProcess
        | ServiceFunction::CloseProcess
        | ServiceFunction::KillProcess
        | ServiceFunction::GetProcessField => {
            services::handle_process_services(guest, &request, plugin);
        }
        _ => {
            println!("Unsupported vmcall function: {:?}", info.func());
            write_response(guest, HypervisorResponse::not_found())
        }
    }
}

pub(crate) fn write_response(guest: &mut dyn Guest, response: HypervisorResponse) {
    guest.regs().r8 = response.arg1;
    guest.regs().r9 = response.arg2;
    guest.regs().r10 = response.arg3;
    guest.regs().rsi = response.result.into_bits() as _;
}

///
/// # Driver Unload
///
/// ## Warning
/// 1. The system WILL stay virtualized!
/// 2. Unloading the driver is unstable and most likely will end in tears.
///
/// TODO: Fix issues above
pub(crate) unsafe extern "C" fn driver_unload(_driver_object: PDRIVER_OBJECT) {}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic occurred: {:?}", _info);

    let param1 = _info
        .message()
        .as_str()
        .unwrap_or("Could not unwrap message");
    let param2 = _info.location().unwrap_or(Location::caller());

    // first parameter is the message.
    // second parameter is the column and line encoded. First 32 bits (LSB) is column, next 32 bits are the line.
    // third parameter is the file location.
    // fourth parameter is reserved.

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
