use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::cell::OnceCell;
use core::fmt::Write;
use log::{Metadata, Record};
use spin::Mutex;
use crate::println;

struct LogBuffer {
    buffer: Vec<u8>,
    cursor: usize,
}

impl LogBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            cursor: 0,
        }
    }

    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.buffer[self.cursor..self.cursor]).unwrap_or("Invalid buffer.")
    }

    pub fn rewind_and_clear(&mut self) {
        self.buffer.fill(0);
        self.cursor = 0;
    }
}

impl Write for LogBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let remaining_len = self.buffer.len() - self.cursor;
        let bytes = s.as_bytes();

        if bytes.len() > remaining_len {
            self.rewind_and_clear();
        }

        self.buffer[self.cursor..self.cursor + bytes.len()].copy_from_slice(bytes);
        Ok(())
    }
}

pub struct NtLogger {
    log_buffer: Mutex<OnceCell<LogBuffer>>,
    pub is_init: bool,
}

unsafe impl Send for NtLogger {}
unsafe impl Sync for NtLogger {}

impl NtLogger {
    pub const fn default() -> Self {
        Self {
            log_buffer: Mutex::new(OnceCell::new()),
            is_init: false,
        }
    }

    pub fn force_get_memory_buffer(&mut self) -> &str {
        // SAFETY: this is executed during a panic. so its "safe" to assume we can force unlock it
        // SAFET: Unless its absolutely init...
        if !self.is_init {
            return "Logger not initialized";
        }

        unsafe { self.log_buffer.force_unlock() };
        let lock = self.log_buffer.get_mut();
        lock.get_mut().unwrap().as_str()
    }

    pub fn init(&mut self) {
        {
            let lock = self.log_buffer.lock();
            let _ = lock.set(LogBuffer::new(4096 * 10));
        }
        self.is_init = true;
    }
}

impl log::Log for NtLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let args = format!(
            "{:>8}: {} ({}, {}:{})\n",
            record.level(),
            record.args(),
            record.target(),
            record.file().unwrap_or("<unknown>"),
            record.line().unwrap_or(0),
        )
        .to_string();

        /*{
            let mut lock = self.serial.lock();
            let _ = lock.get_mut().unwrap().write_str(args.as_str());
        }*/
        {
            let mut lock = self.log_buffer.lock();
            let _ = lock.get_mut().unwrap().write_str(args.as_str());
        }

        println!("{}", args);
    }

    fn flush(&self) {}
}
