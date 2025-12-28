#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;

mod cback;
mod nt;
mod ops;
mod plugins;
mod services;
mod utils;
mod win;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use crate::cback::registry_callback;
use crate::plugins::PluginTable;
use alloc::boxed::Box;
use core::mem;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use hv::SharedHostData;
use hv::hypervisor::host::Guest;
use hxposed_core::hxposed::call::{HypervisorCall, ServiceParameter};
use hxposed_core::hxposed::error::{NotAllowedReason, NotFoundReason};
use hxposed_core::hxposed::func::ServiceFunction;
use hxposed_core::hxposed::func::ServiceFunction::Authorize;
use hxposed_core::hxposed::requests::auth::AuthorizationRequest;
use hxposed_core::hxposed::requests::{HypervisorRequest, VmcallRequest};
use hxposed_core::hxposed::responses::status::StatusResponse;
use hxposed_core::hxposed::responses::{HypervisorResponse, VmcallResponse};
use hxposed_core::hxposed::status::HypervisorStatus;
use hxposed_core::services::async_service::UnsafeAsyncInfo;
use log::LevelFilter;

use utils::logger::NtLogger;
use crate::nt::worker::async_worker_thread;
use crate::services::authorize_plugin;
use wdk_alloc::WdkAllocator;
use wdk_sys::ntddk::{
    CmRegisterCallback, KeBugCheckEx, ProbeForRead, PsCreateSystemThread, ZwAllocateVirtualMemory,
    ZwClose,
};
use wdk_sys::{
    BOOLEAN, DRIVER_OBJECT, HANDLE, LARGE_INTEGER, MEM_COMMIT, MEM_RESERVE, NTSTATUS,
    PAGE_READWRITE, PUNICODE_STRING, PVOID, SIZE_T, STATUS_SUCCESS, STATUS_TOO_LATE,
    THREAD_ALL_ACCESS,
};
use crate::nt::mdl::{MapStatus, MemoryDescriptor};

static CM_COOKIE: AtomicU64 = AtomicU64::new(0);
static PLUGINS: AtomicPtr<PluginTable> = AtomicPtr::new(null_mut());
static LOGGER: NtLogger = NtLogger;

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
extern "C" fn driver_entry(
    _driver: &mut DRIVER_OBJECT,
    _registry_path: PUNICODE_STRING,
    hxloader: BOOLEAN,
) -> NTSTATUS {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    log::info!("Welcome to HxPosed!");

    match hxloader == 1 {
        true => {
            log::info!("Loaded from HxLoader!");
            nt::get_nt_info(Some(26200)); // for some reason, its 26100. but since its from HxLoader, we know its 25h2.
        }
        false => {
            nt::get_nt_info(None);
        }
    }

    nt::get_system_token();

    if nt::NT_BUILD.load(Ordering::Relaxed) != 26200 {
        log::error!("Unsupported version");
        return STATUS_TOO_LATE;
    }

    log::info!("HxPosed Initialized.");
    log::info!("NT Version: {:x}", nt::NT_BUILD.load(Ordering::Relaxed));
    log::info!(
        "SYSTEM Token: {:x}",
        nt::SYSTEM_TOKEN.load(Ordering::Relaxed) as u64
    );

    log::info!("Allocating memory for the hypervisor...");

    let mut hv_mem = MemoryDescriptor::new(hv::allocator::ALLOCATION_BYTES);
    hv_mem.map(None);

    let ptr = match hv_mem.status {
        MapStatus::Mapped(ptr) => ptr,
        MapStatus::Allocated => {
            log::error!("Failed to allocate memory for hypervisor.");
            panic!();
        }
        MapStatus::NotInitialized => unreachable!()
    };

    log::info!("Allocated {:x} bytes and mapped to {:x}", hv_mem.length, ptr);

    hv::allocator::init(ptr as _);

    mem::forget(hv_mem); // this memory should not be dropped. it will live as long as the system does

    hv::platform_ops::init(Box::new(ops::WindowsOps));

    // TODO: use custom gdt and so on for more security?
    let mut host_data = SharedHostData::default();
    host_data.vmcall_handler = Some(vmcall_handler);

    hv::virtualize_system(host_data);

    plugins::load_plugins();

    let mut cookie = LARGE_INTEGER::default();
    match unsafe { CmRegisterCallback(Some(registry_callback), PVOID::default(), &mut cookie) } {
        STATUS_SUCCESS => unsafe { CM_COOKIE.store(cookie.QuadPart as _, Ordering::Relaxed) },
        err => {
            log::error!("Error registering registry callbacks: {:?}", err);
            panic!("Panicking for your own good");
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
            let _ = ZwClose(handle);
        },
        err => {
            log::error!("Error creating worker thread: {:?}", err);
            panic!("Panicking for your own good");
        }
    }

    STATUS_SUCCESS
}

///
/// # Called when a CPUID with RCX = 2009 is executed.
///
/// ## Arguments
/// guest - The trait of guest. Intel or AMD.
///
/// info - Information about the call. See [HypervisorCall].
///
/// ## Return
/// There is no return value of this function, however, the return value of the vmcall will be in RSI.
/// Which you *may* want to utilize. See documentation on GitHub page for more information about trap ABI.
///
/// ## Warning
///
/// ### We are in context of the thread that made the vmcall.
/// Functions like "IoGetCurrentProcess" returns the process that made the vmcall, not the system process.
/// (that is a good thing)
///
/// ### IRQL is above sane.
/// IRQL is 255, all interrupts are disabled. Using Zw* and other functions that ask for PASSIVE_LEVEL will only result in tears.
///
/// ### This is a VMEXIT handler
/// Don't you dare to "take your time". This interrupts the whole CPU and making the kernel scheduler forget its purpose.
///
fn vmcall_handler(guest: &mut dyn Guest, info: HypervisorCall) {
    log::trace!("Handling vmcall function: {:?}", info.func());

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

    let mut async_info = UnsafeAsyncInfo::default();

    let request = HypervisorRequest {
        call: info,
        arg1: guest.regs().r8,
        arg2: guest.regs().r9,
        arg3: guest.regs().r10,
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
                // TODO: Validate this handle? How?
                async_info = UnsafeAsyncInfo {
                    handle: guest.regs().r11,
                    result_values: guest.regs().r12 as *mut _, // rsi, r8, r9, r10. total 4
                };

                log::trace!(
                    "Async Handle: {:x}. Result values: {:x}",
                    async_info.handle,
                    async_info.result_values.addr()
                )
            }
            Err(_) => {
                log::warn!("Invalid async buffer provided by user.");
                write_response(
                    guest,
                    HypervisorResponse::invalid_params(ServiceParameter::BufferByUser),
                );
                return;
            }
        }
    }

    let plugin = match PluginTable::current() {
        None => {
            log::trace!("Plugin is not authorized.");
            if info.func() == Authorize {
                log::trace!("Authorizing...");
                authorize_plugin(guest, AuthorizationRequest::from_raw(&request));
                return;
            }
            log::warn!("Plugin tried to use HxPosed without authorizing first.");
            write_response(
                guest,
                HypervisorResponse::not_allowed(NotAllowedReason::PluginNotLoaded),
            );
            return;
        }
        Some(x) => x,
    };

    // we could actually use a bit mask that defines which category the service belongs to.
    // so we would spare ourselves from checking the func 2 times.
    // but rust enums aren't that easy, so we got this.
    // TODO: do what I said.
    match info.func() {
        ServiceFunction::OpenProcess
        | ServiceFunction::CloseProcess
        | ServiceFunction::KillProcess
        | ServiceFunction::GetProcessField
        | ServiceFunction::SetProcessField
        | ServiceFunction::GetProcessThreads => {
            services::handle_process_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::OpenThread
        | ServiceFunction::CloseThread
        | ServiceFunction::SuspendResumeThread
        | ServiceFunction::KillThread
        | ServiceFunction::GetThreadField
        | ServiceFunction::SetThreadField => {
            services::handle_thread_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::ProcessVMOperation
        | ServiceFunction::ProtectProcessMemory
        | ServiceFunction::AllocateMemory
        | ServiceFunction::MapMemory
        | ServiceFunction::FreeMemory => {
            services::handle_memory_services(guest, &request, plugin, async_info);
        }
        ServiceFunction::OpenToken
        | ServiceFunction::CloseToken
        | ServiceFunction::GetTokenField
        | ServiceFunction::SetTokenField => {
            services::handle_security_services(guest, &request, plugin, async_info);
        }
        _ => {
            log::warn!("Unsupported vmcall: {:?}", info.func());
            write_response(
                guest,
                HypervisorResponse::not_found_what(NotFoundReason::ServiceFunction),
            )
        }
    }
}

pub(crate) fn write_response(guest: &mut dyn Guest, response: HypervisorResponse) {
    guest.regs().r8 = response.arg1;
    guest.regs().r9 = response.arg2;
    guest.regs().r10 = response.arg3;
    guest.regs().rsi = response.result.into_bits() as _;
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    log::error!("Panic occurred: {:?}", _info);
    log::error!("Take a moment and report this to us in GitHub.");
    log::error!("- Yours truly.");

    let param1 = _info
        .message()
        .as_str()
        .unwrap_or("Could not unwrap message");

    unsafe {
        KeBugCheckEx(0x2009, param1.as_ptr() as _, 0, 0, 0);
    }
}
