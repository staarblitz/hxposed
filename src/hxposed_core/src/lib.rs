#![no_std]

use core::panic::PanicInfo;

mod error;
pub mod hxposed;
mod intern;
pub mod svcs;

#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
