#![feature(unboxed_closures)]
#![feature(allocator_api)]
#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;
extern crate bit_field;

mod boot;
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
use crate::nt::NT_BUILD;
use crate::nt::guard::hxguard::HxGuard;
use crate::nt::thread::NtThread;
use crate::utils::logger::{HxLogger, LogEvent};
use crate::win::winalloc::WdkAllocator;
use crate::win::{
    Boolean, KeBugCheckEx, KeDelayExecutionThread, KeGetCurrentProcessorNumber, NtStatus, PVOID,
    ProcessorMode,
};
use alloc::format;
use core::ops::DerefMut;
use core::ptr;
use core::ptr::null_mut;
use spin::{Lazy, Mutex};

static mut HX_GUARD: HxGuard = HxGuard::new();
static GLOBAL_LOGGER: Lazy<Mutex<HxLogger>> = Lazy::new(|| Mutex::new(HxLogger::new()));
#[unsafe(no_mangle)]
static mut GLOBAL_LOGGER_PTR: u64 = 0;

#[unsafe(link_section = "INIT")]
#[unsafe(export_name = "DriverEntry")]
#[allow(static_mut_refs)]
extern "C" fn driver_entry(_driver: PVOID, _registry_path: PVOID) -> NtStatus {
    // SAFETY: we know its aligned and safe to read.
    let cfg = unsafe {
        // must use read_volatile so rust compiler doesn't assume things.
        ptr::read_volatile(&HX_LOADER_PARAMETER_BLOCK)
    };

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

    nt::arch::hijack_pcrs();

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
    let msg = format!("{:?}", info);

    unsafe {
        GLOBAL_LOGGER.force_unlock();
        let mut logger = GLOBAL_LOGGER.lock();
        logger.error(LogEvent::Panic(0_, msg.as_ptr() as _));
    }

    unsafe {
        KeBugCheckEx(
            0x2009,
            msg.as_ptr() as _,
            KeGetCurrentProcessorNumber() as _,
            &HX_LOADER_PARAMETER_BLOCK as *const _ as _,
            0
        );
    };
}
