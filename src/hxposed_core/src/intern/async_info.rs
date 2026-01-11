#![allow(dead_code)]

#[derive(Default, Debug)]
pub struct UnsafeAsyncInfo {
    pub handle: u64,
    pub result_values: *mut [u64; 4],
}

impl UnsafeAsyncInfo {
    pub fn is_present(&self) -> bool {
        self.handle != 0
    }
}
