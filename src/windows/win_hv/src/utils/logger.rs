use alloc::string::ToString;
use com_logger::Serial;
use core::cell::OnceCell;
use core::fmt::Write;
use log::{Metadata, Record};
use spin::Mutex;
use wdk::{print};

pub struct NtLogger {
    serial: Mutex<OnceCell<Serial>>,
    pub is_init: bool,
}

unsafe impl Send for NtLogger {}
unsafe impl Sync for NtLogger {}

impl NtLogger {
    pub const fn default() -> Self {
        Self {
            serial: Mutex::new(OnceCell::new()),
            is_init: false,
        }
    }

    pub fn init(&mut self) {
        let lock = self.serial.lock();
        let _ = lock.set(Serial::new(0x3f8));
        self.is_init = true;
    }
}

impl log::Log for NtLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let args = format_args!(
            "{:>8}: {} ({}, {}:{})\n",
            record.level(),
            record.args(),
            record.target(),
            record.file().unwrap_or("<unknown>"),
            record.line().unwrap_or(0),
        )
        .to_string();

        {
            let mut lock = self.serial.lock();
            let _ = lock.get_mut().unwrap().write_str(args.as_str());
        }

        print!("{}", args);
    }

    fn flush(&self) {}
}
