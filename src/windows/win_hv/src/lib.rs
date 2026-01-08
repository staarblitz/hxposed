#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;

mod nt;
mod ops;
mod plugins;
mod services;
mod utils;
mod win;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use nt::cback::registry_callback;
use crate::nt::mdl::{MapStatus, MemoryDescriptor};
use crate::nt::thread::NtThread;
use crate::nt::worker::async_worker_thread;
use crate::plugins::PluginTable;
use crate::services::authorize_plugin;
use crate::utils::logger::NtLogger;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString;
use core::mem;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU64, Ordering};
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
use wdk_alloc::WdkAllocator;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{CmRegisterCallback, ExAllocatePool2, KeBugCheckEx, KeDelayExecutionThread};
use wdk_sys::{BOOLEAN, DRIVER_OBJECT, FALSE, LARGE_INTEGER, NTSTATUS, POOL_FLAG_NON_PAGED, PUNICODE_STRING, PVOID, STATUS_SUCCESS, STATUS_TOO_LATE};

static CM_COOKIE: AtomicU64 = AtomicU64::new(0);
static mut LOGGER: NtLogger = NtLogger::default();

extern "C" fn delayed_start(arg: PVOID) {
    // im very sorry

    log::trace!("delayed_start");

    let mut time = utils::timing::relative(utils::timing::seconds(20));

    let _ = unsafe{
        KeDelayExecutionThread(
            KernelMode as _,
            FALSE as _,
            &mut time as *mut _ as _
        )
    };

    log::info!("Delayed! Executing real entry!");

    driver_entry(null_mut(), null_mut(), 0);
}

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
#[allow(static_mut_refs)]
extern "C" fn driver_entry(
    _driver: *mut DRIVER_OBJECT,
    _registry_path: PUNICODE_STRING,
    hxloader: BOOLEAN,
) -> NTSTATUS {
    // SAFETY: we know there is no other accessor currently
    unsafe {
        if !LOGGER.is_init {
            LOGGER.init();

            //log::set_logger_force(|| &LOGGER);
            let _ = log::set_logger(&LOGGER);
            log::set_max_level(log::LevelFilter::Trace);
        }
    }

    log::trace!("driver_entry");
    log::info!("Welcome to HxPosed!");

    match hxloader == 1 {
        true => {
            log::info!("Loaded from HxLoader!");
            log::info!("Delaying startup....");

            NtThread::create(Some(delayed_start), None);

            return STATUS_SUCCESS;
        }
        false => {
            match nt::get_nt_info(Some(26200)) {
                Ok(_) => {}
                Err(_) => {
                    log::error!("System is not virtualized.");
                    return STATUS_TOO_LATE;
                }
            }
        }
    }

    log::info!("Initializing HxPosed");

    nt::get_system_token();

    log::info!("NT Version: {:x}", nt::NT_BUILD.load(Ordering::Relaxed));
    log::info!(
        "SYSTEM Token: {:x}",
        nt::SYSTEM_TOKEN.load(Ordering::Relaxed) as u64
    );

    log::info!("Allocating memory for the hypervisor...");

    let mem = unsafe {
        ExAllocatePool2(POOL_FLAG_NON_PAGED, hv::allocator::ALLOCATION_BYTES as _, 0x2009)
    };

    hv::allocator::init(mem as _);

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
            panic!("Error registering registry callbacks: {:?}", err);
        }
    }

    match NtThread::create(Some(async_worker_thread), None) {
        STATUS_SUCCESS => {}
        _ => {
            panic!();
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
        match nt::probe::probe_for_write(guest.regs().r12 as _, 16, 1) {
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

    let param1 = format!("{:?}", _info);

    unsafe {
        KeBugCheckEx(0x2009, param1.as_ptr() as _, 0, 0, 0);
    };
}
