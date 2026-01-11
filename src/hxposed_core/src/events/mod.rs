use alloc::boxed::Box;
use spin::Mutex;

#[cfg(feature = "usermode")]
pub mod async_service;

#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    pub result_values: Mutex<Box<[u64; 4]>>,
}