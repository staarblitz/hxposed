use alloc::{string::ToString, vec::Vec};
use wdk_sys::{
    UNICODE_STRING,
    ntddk::{RtlFreeUnicodeString, RtlInitUnicodeString},
};

pub struct UnicodeString {
    buffer: Vec<u16>,
}

impl UnicodeString {
    pub fn new(data: &str) -> Self {
        let mut buffer: Vec<u16> = data.encode_utf16().collect();
        Self { buffer }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.buffer.as_ptr()
    }

    pub fn get_raw_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts::<u8>(self.as_ptr() as _, self.len() * 2) }
    }

    pub fn from_unicode_string(str: &UNICODE_STRING) -> Self {
        let mut vec: Vec<u16> = Vec::with_capacity((str.MaximumLength / 2) as _);

        unsafe {
            // not Vec::from_raw_parts because it does not copy, it owns the buffer
            core::ptr::copy_nonoverlapping(str.Buffer, vec.as_mut_ptr(), (str.Length / 2) as _);
            vec.set_len((str.Length / 2) as _);
        }
        Self { buffer: vec }
    }

    pub fn contains(&self, new: &str) -> bool {
        let new_buffer: Vec<u16> = new.encode_utf16().collect();
        self.buffer
            .windows(new_buffer.len())
            .find(|x| *x == new_buffer)
            .is_some()
    }

    pub fn concat(&mut self, new: &str) {
        let new_buffer: Vec<u16> = new.encode_utf16().collect();
        self.buffer.extend(new_buffer);
    }

    pub fn concat_from_unicode_string(&mut self, new: &UnicodeString) {
        let new_buffer: Vec<u16> = new.buffer.clone();
        self.buffer.extend(new_buffer);
    }

    pub fn to_unicode_string<'a>(&mut self) -> UNICODE_STRING {
        UNICODE_STRING {
            Buffer: self.buffer.as_mut_ptr(),
            Length: (self.buffer.len() * 2) as _,
            MaximumLength: (self.buffer.capacity() * 2) as _,
        }
    }
}
