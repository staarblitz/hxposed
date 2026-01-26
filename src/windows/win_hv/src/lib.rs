#![feature(unboxed_closures)]
#![no_std]

extern crate alloc;
extern crate bit_field;
extern crate hv;

mod boot;
mod hypervisor;
mod nt;
mod objects;
mod services;
mod utils;
mod win;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use crate::boot::HX_LOADER_PARAMETER_BLOCK;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::object::NtObject;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::utils::logger::NtLogger;
use core::ptr;
use core::ptr::null_mut;
use core::sync::atomic::Ordering;
use wdk_alloc::WdkAllocator;
use wdk_sys::_MODE::KernelMode;
use wdk_sys::ntddk::{KeBugCheckEx, KeDelayExecutionThread};
use wdk_sys::{
    DRIVER_OBJECT, FALSE, NTSTATUS, PUNICODE_STRING, PVOID, STATUS_SUCCESS, STATUS_TOO_LATE,
};
use crate::objects::async_obj::AsyncState;

static mut HX_GUARD: HxGuard = HxGuard::new();

static mut LOGGER: NtLogger = NtLogger::default();

extern "C" fn delayed_start(_arg: PVOID) {
    // im very sorry

    log::trace!("delayed_start");

    let mut time = utils::timing::relative(utils::timing::seconds(20));

    let _ =
        unsafe { KeDelayExecutionThread(KernelMode as _, FALSE as _, &mut time as *mut _ as _) };

    log::info!("Delayed! Executing real entry!");

    driver_entry(null_mut(), null_mut());
}

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
#[allow(static_mut_refs)]
extern "C" fn driver_entry(
    _driver: *mut DRIVER_OBJECT,
    _registry_path: PUNICODE_STRING,
) -> NTSTATUS {
    // SAFETY: we know its aligned and safe to read.
    let cfg = unsafe {
        // must use read_volatile so rust compiler doesn't assume things.
        ptr::read_volatile(&HX_LOADER_PARAMETER_BLOCK)
    };

    match cfg.booted_from_hxloader
     && !_driver.is_null() && !_registry_path.is_null() /* Make sure we are called from delayed_start */
     {
        true => {
            log::info!("Loaded from HxLoader!");
            log::info!("Delaying startup....");

            NtThread::create(Some(delayed_start), None);

            return STATUS_SUCCESS;
        }
        false => {}
    }

    match nt::get_nt_info() {
        Err(_) => return STATUS_TOO_LATE,
        Ok(_) => {}
    }

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

    log::info!(
        "HxPosed base {:x}, size {:x}",
        cfg.base_address,
        cfg.pe_size
    );

    log::info!("NT Version: {:x}", nt::NT_BUILD.load(Ordering::Relaxed));
    log::info!(
        "SYSTEM Token: {:x}",
        nt::SYSTEM_TOKEN.load(Ordering::Relaxed) as u64
    );

    log::info!("Initializing HxPosed");

    // SAFETY: this is the only mutable access in entire lifecycle.
    unsafe {
        HX_GUARD.init();
    }

    log::info!("Initializing hypervisor...");

    hypervisor::init::init_hypervisor();

    match nt::callback::NtCallback::init() {
        Ok(_) => {}
        Err(err) => {
            panic!("Failed to initialize callbacks: {:x}", err);
        }
    }

    STATUS_SUCCESS
}

#[panic_handler]
#[allow(static_mut_refs)]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("Panic occurred: {:?}", info);

    unsafe {
        KeBugCheckEx(
            0x2009,
            info.message().as_str().unwrap_or("Lol").as_ptr() as _,
            LOGGER.force_get_memory_buffer().as_ptr() as _,
            0,
            0,
        );
    };
}
