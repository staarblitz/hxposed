use log::{Metadata, Record};
use wdk::println;

pub struct NtLogger;

impl log::Log for NtLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!(
            "[HXPOSED] [{}] - {} ({}@{})",
            record.level(),
            record.args(),
            record.file().unwrap_or("No file"),
            record.line().unwrap_or(0)
        );
    }

    fn flush(&self) {}
}
