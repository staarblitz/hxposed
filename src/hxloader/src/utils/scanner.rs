use core::ptr::slice_from_raw_parts;

pub struct Scanner;

impl Scanner {
    pub fn pattern_scan(base: *const u8, len: usize, pattern: &[u8]) -> Option<*const u8> {
        let slice = unsafe { slice_from_raw_parts(base, len).as_ref().unwrap() };

        slice
            .windows(pattern.len())
            .find(|window| *window == pattern)
            .map(|window| window.as_ptr())
    }
}
