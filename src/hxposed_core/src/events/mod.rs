use alloc::boxed::Box;
use spin::Mutex;
use crate::hxposed::ProcessObject;

#[derive(Default, Debug)]
pub struct UnsafeAsyncInfo {
    pub handle: u64,
    pub result_values: *mut [u64; 4],
    pub process: ProcessObject
}

unsafe impl Send for UnsafeAsyncInfo {}
unsafe impl Sync for UnsafeAsyncInfo {}

impl UnsafeAsyncInfo {
    pub fn is_present(&self) -> bool {
        self.handle != 0
    }
}


#[cfg(feature = "usermode")]
pub mod async_service;

#[derive(Default, Debug)]
pub struct AsyncInfo {
    pub handle: u64,
    pub result_values: Mutex<Box<[u64; 4]>>,
}