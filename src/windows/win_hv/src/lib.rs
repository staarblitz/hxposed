#![feature(unboxed_closures)]
#![feature(allocator_api)]
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

#[unsafe(no_mangle)]
static __CxxFrameHandler3: u32 = 0;

#[unsafe(no_mangle)]
static _fltused: u32 = 0;

#[global_allocator]
static GLOBAL_ALLOC: WdkAllocator = WdkAllocator;

use crate::boot::HX_LOADER_PARAMETER_BLOCK;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::thread::NtThread;
use crate::win::winalloc::WdkAllocator;
use crate::win::{Boolean, KeDelayExecutionThread, NtStatus, PVOID, ProcessorMode, KeBugCheckEx};
use core::ptr;
use core::ptr::null_mut;
use core::sync::atomic::Ordering;
use crate::utils::logger::NtLogger;

static mut HX_GUARD: HxGuard = HxGuard::new();

static mut LOGGER: NtLogger = NtLogger::default();

extern "C" fn delayed_start(_arg: PVOID) {
    // im very sorry

    log::trace!("delayed_start");

    let mut time = utils::timing::relative(utils::timing::seconds(20));

    let _ = unsafe { KeDelayExecutionThread(ProcessorMode::KernelMode, Boolean::False, &mut time) };

    log::info!("Delayed! Executing real entry!");

    driver_entry(null_mut(), null_mut());
}

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
#[allow(static_mut_refs)]
extern "C" fn driver_entry(_driver: PVOID, _registry_path: PVOID) -> NtStatus {
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

            match NtThread::create(delayed_start, None) {
                Ok(_) => {}
                Err(err) => {
                    panic!("Failed to create thread: {}", err);
                }
            };

            return NtStatus::Success;
        }
        false => {}
    }

    match nt::get_nt_info() {
        Err(_) => return NtStatus::TooLate,
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
            panic!("Failed to initialize callbacks: {}", err);
        }
    }

    NtStatus::Success
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
