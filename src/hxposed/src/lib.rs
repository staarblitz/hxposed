#![feature(unboxed_closures)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;
extern crate bit_field;

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
use crate::hypervisor::vcpu::Vmcs;
use crate::nt::NT_BUILD;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::thread::NtThread;
use crate::utils::logger::{HvLogger, LogEvent};
use crate::win::winalloc::WdkAllocator;
use crate::win::{
    Boolean, KeBugCheckEx, KeDelayExecutionThread, KeGetCurrentProcessorNumber, NtStatus, PVOID,
    ProcessorMode,
};
use alloc::format;
use core::ops::DerefMut;
use core::ptr;
use core::ptr::null_mut;
use core::sync::atomic::Ordering;
use spin::{Lazy, Mutex};

static mut HX_GUARD: HxGuard = HxGuard::new();
static GLOBAL_LOGGER: Lazy<Mutex<HvLogger>> = Lazy::new(|| Mutex::new(HvLogger::new()));
#[unsafe(no_mangle)]
static mut GLOBAL_LOGGER_PTR: u64 = 0;

extern "C" fn delayed_start(_arg: PVOID) {
    // im very sorry

    let mut logger = GLOBAL_LOGGER.lock();

    logger.info(LogEvent::DelayedStart);

    let mut time = utils::timing::relative(utils::timing::seconds(20));

    let _ = unsafe { KeDelayExecutionThread(ProcessorMode::KernelMode, Boolean::False, &mut time) };

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

    unsafe {
        let mut locked = GLOBAL_LOGGER.lock();
        GLOBAL_LOGGER_PTR = locked.deref_mut() as *mut _ as u64;
    }

    match nt::get_nt_info() {
        Err(_) => return NtStatus::TooLate,
        Ok(_) => {}
    }

    scoped_log!(info, LogEvent::HxPosedInit(cfg.base_address, cfg.pe_size));

    // SAFETY: this is the only mutable access in entire lifecycle.
    unsafe {
        HX_GUARD.init();
    }

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
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    let dump = Vmcs::dump();
    let msg = format!("{:?}", info);

    unsafe {
        GLOBAL_LOGGER.force_unlock();
        let mut logger = GLOBAL_LOGGER.lock();
        logger.error(LogEvent::Panic(dump.as_ptr() as _, msg.as_ptr() as _));
    }

    unsafe {
        KeBugCheckEx(
            0x2009,
            msg.as_ptr() as _,
            dump.as_ptr() as _,
            KeGetCurrentProcessorNumber() as _,
            &HX_LOADER_PARAMETER_BLOCK as *const _ as _,
        );
    };
}
