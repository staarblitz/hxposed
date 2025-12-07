use log::{Metadata, Record};
use wdk::println;

pub struct NtLogger;

impl log::Log for NtLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("[HXPOSED] [{}] - {}", record.level(), record.args());
    }

    fn flush(&self) {

    }
}