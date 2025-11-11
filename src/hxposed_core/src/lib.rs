#![no_std]
extern crate alloc;

use core::panic::PanicInfo;

mod error;
pub mod hxposed;
mod intern;
pub mod svcs;
mod nt;
mod plugins;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
