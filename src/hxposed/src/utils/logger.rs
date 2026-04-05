use crate::win::{KeGetCurrentProcessorNumber, KeQuerySystemTime};
use alloc::boxed::Box;
use spin::mutex::SpinMutex;
use x86::io::outb;
use crate::size_assert;

#[repr(C)]
pub struct HxLogger {
    buffer: Box<[LogEntry; 4096 * 8]>,
    cursor: usize,
    cycle: u32
}

static SERIAL_LOCK: SpinMutex<()> = SpinMutex::new(());

impl HxLogger {

    pub fn new() -> Self {
        Self {
            buffer: unsafe { Box::new_zeroed().assume_init() },
            cursor: 0,
            cycle: 0
        }
    }

    fn serial_out(entry: &LogEntry) {
        let _lock = SERIAL_LOCK.lock();
        // SAFETY: the struct is exactly 64 bytes and aligned
        let ptr = entry as *const LogEntry as *const u8;

        for i in 0..64 {
            unsafe {
                let byte = *ptr.add(i);
                outb(0xe9, byte);
            }
        }

        drop(_lock);
    }

    pub fn dump_to_serial(&self) {
        for i in 0..self.cursor {
            Self::serial_out(&self.buffer[i]);
        }
    }

    pub fn serial_log(log_type:LogType, event: LogEvent) {
        Self::serial_out(&LogEntry::new(log_type, event))
    }

    pub fn log(&mut self, log_type: LogType, event: LogEvent) {
        if self.cursor >= self.buffer.len() {
            self.cursor = 1;
            self.cycle += 1;
            self.buffer[0] = LogEntry::new(LogType::Info, LogEvent::LogRingReset(self.cycle));
        }

        self.buffer[self.cursor] = LogEntry::new(log_type, event);
        Self::serial_out(&self.buffer[self.cursor]);
        self.cursor += 1;
    }

    pub fn trace(&mut self, event: LogEvent) {
        self.log(LogType::Trace, event);
    }

    pub fn info(&mut self, event: LogEvent) {
        self.log(LogType::Info, event);
    }

    pub fn warn(&mut self, event: LogEvent) {
        self.log(LogType::Warn, event);
    }

    pub fn error(&mut self, event: LogEvent) {
        self.log(LogType::Error, event);
    }
}

#[repr(C, align(64))]
#[derive(Default, Copy, Clone)]
pub struct LogEntry {
    pub log_type: LogType,
    pub processor: u64,
    pub timestamp: u64,
    pub discriminant: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
}
size_assert!(LogEntry, 64);

impl LogEntry {
    pub fn new(log_type: LogType, event: LogEvent) -> Self {
        let raw = event.into_raw();
        Self {
            log_type,
            discriminant: raw.0,
            arg1: raw.1,
            arg2: raw.2,
            arg3: raw.3,
            arg4: raw.4,
            processor: unsafe { KeGetCurrentProcessorNumber() as _ },
            timestamp: unsafe { KeQuerySystemTime() },
        }
    }
}

#[repr(u64)]
#[derive(Copy, Clone, Default)]
pub enum LogEvent {
    #[default]
    None = 0,
    AcquireObject(u64, u64) = 1,
    FreeObject(u64, u64) = 2,
    NoHxInfo = 5,
    QueryObject(u64, u64) = 3,
    TrackObject(u64, u64) = 4,
    DetrackObject(u64, u64) = 10,
    SystemCall(u64, u64, u64, u64) = 6,
    SystemDispatch(u64, u64) = 7,
    CallResult(u64, u64, u64) = 9,
    FailedToMap = 12,
    HxPosedInit(u64, u64) = 14,
    FailedToAllocate = 17,
    Exception(u32) = 18,
    Catastrophic(u64, u32, u32) = 19,
    IncrementRefCount(u64, u64) = 20,
    DecrementRefCount(u64, u64) = 21,
    IncrementHandleCount(u64, u64) = 22,
    DecrementHandleCount(u64, u64) = 23,
    Panic(u64, u64) = 25,
    NtInfo(u64, u64, u64) = 28,
    BuildOffset(u32, u64) = 29,
    LogRingReset(u32) = 30
}

impl LogEvent {
    // generally, in logging, goal is to save as much space as possible. but since rust enums are ass we have to do this
    pub const fn into_raw(self) -> (u64, u64, u64, u64, u64) {
        match self {
            LogEvent::None => (0, 0, 0, 0, 0),
            LogEvent::TrackObject(x,y) => (3, x, y, 0,0),
            LogEvent::QueryObject(x,y) => (4, x, y, 0,0),
            LogEvent::DetrackObject(x, y) => (5, x, y, 0,0),
            LogEvent::AcquireObject(x, y) => (1, x, y, 0, 0),
            LogEvent::FreeObject(x,y) => (2, x, y, 0, 0),
            LogEvent::NoHxInfo => (5, 0, 0, 0, 0),
            LogEvent::SystemCall(x, y, z, q) => (6, x, y, z, q),
            LogEvent::SystemDispatch(x, y) => (7, x, y, 0, 0),
            LogEvent::CallResult(x, y, z) => (9, x, y, z, 0),
            LogEvent::FailedToMap => (12, 0, 0, 0, 0),
            LogEvent::HxPosedInit(x, y) => (14, x, y, 0, 0),
            LogEvent::FailedToAllocate => (17, 0, 0, 0, 0),
            LogEvent::Exception(x) => (18, x as _, 0, 0, 0),
            LogEvent::Catastrophic(x, y, z) => (19, x, y as _, z as _, 0),
            LogEvent::Panic(x, y) => (25, x, y, 0, 0),
            LogEvent::NtInfo(x, y, z) => (28, x, y, z, 0),
            LogEvent::BuildOffset(x,y) => (29, x as _, y, 0, 0),
            LogEvent::LogRingReset(x) => (30, x as _,0,0,0),
            LogEvent::IncrementRefCount(x,y) => (20, x as _, y as _, 0, 0),
            LogEvent::DecrementRefCount(x,y) => (21, x as _, y as _, 0, 0),
            LogEvent::IncrementHandleCount(x,y) => (22, x as _, y as _, 0, 0),
            LogEvent::DecrementHandleCount(x,y) => (23, x as _, y as _, 0, 0),
        }
    }
}

#[repr(u64)]
#[derive(Copy, Clone, Default)]
pub enum LogType {
    #[default]
    Trace = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}
