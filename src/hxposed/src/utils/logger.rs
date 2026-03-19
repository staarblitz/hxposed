use crate::win::{KeGetCurrentProcessorNumber, KeQuerySystemTime};
use alloc::boxed::Box;
use x86::io::outb;

pub struct HvLogger {
    buffer: Box<[LogEntry; 4096 * 8]>,
    cursor: usize,
}

impl HvLogger {
    pub fn new() -> Self {
        Self {
            buffer: unsafe { Box::new_zeroed().assume_init() },
            cursor: 0,
        }
    }

    // <qemu:arg value="-chardev"/>
    // <qemu:arg value="file,id=debuglog,path=/tmp/guest_debug.log,append=on"/>
    // <qemu:arg value="-device"/>
    // <qemu:arg value="isa-debugcon,iobase=0xe9,chardev=debuglog"/>
    fn serial_out(entry: &LogEntry) {
        // SAFETY: the struct is exactly 64 bytes and aligned
        let ptr = entry as *const LogEntry as *const u8;

        for i in 0..64 {
            unsafe {
                let byte = *ptr.add(i);
                outb(0xe9, byte);
            }
        }
    }

    pub fn dump_to_serial(&self) {
        for i in 0..self.cursor {
            Self::serial_out(&self.buffer[i]);
        }
    }

    pub fn log(&mut self, log_type: LogType, event: LogEvent) {
        if self.cursor >= self.buffer.len() {
            self.cursor = 0;
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

#[repr(C, align(8))]
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
const _: () = assert!(size_of::<LogEntry>() == 64);

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
    VmxExitReason(u64),
    RIP(u64, u64) = 2,
    VCPU(u64) = 3,
    UnknownExitReason(u64) = 4,
    NoHxInfo = 5,
    HyperCall(u64, u64, u64, u64) = 6,
    HcDispatch(u64, u64) = 7,
    CallError(u64, u64) = 8,
    HyperResult(u64, u64, u64) = 9,
    VirtualizingProcessor(u32) = 10,
    ProcessorVirtualized(u32) = 11,
    FailedToMap = 12,
    DelayedStart = 13,
    HxPosedInit(u64, u64) = 14,
    NtVersion(u64) = 15,
    WindowsVersion(u32, u32) = 16,
    FailedToAllocate = 17,
    Exception(u32) = 18,
    Catastrophic(u64, u32, u32) = 19,
    ProcessorReady(u32, u64) = 20,
    LaunchingProcessor = 21,
    Vmclear(u64, u64) = 22,
    Vmptrld(u64, u64) = 23,
    Vmxon(u64, u64) = 24,
    Panic(u64, u64) = 25,
    WritingAsyncBuffer(u64, u64) = 26,
    WrittenAsyncBuffer(u64, u64) = 27,
}

impl LogEvent {
    // generally, in logging, goal is to save as much space as possible. but since rust enums are ass we have to do this
    pub const fn into_raw(self) -> (u64, u64, u64, u64, u64) {
        match self {
            LogEvent::None => (0, 0, 0, 0, 0),
            LogEvent::VmxExitReason(x) => (1, x, 0, 0, 0),
            LogEvent::RIP(x, y) => (2, x, y, 0, 0),
            LogEvent::VCPU(x) => (3, x, 0, 0, 0),
            LogEvent::UnknownExitReason(x) => (4, x, 0, 0, 0),
            LogEvent::NoHxInfo => (5, 0, 0, 0, 0),
            LogEvent::HyperCall(x, y, z, q) => (6, x, y, z, q),
            LogEvent::HcDispatch(x, y) => (7, x, y, 0, 0),
            LogEvent::CallError(x, y) => (8, x, y, 0, 0),
            LogEvent::HyperResult(x, y, z) => (9, x, y, z, 0),
            LogEvent::VirtualizingProcessor(x) => (10, x as _, 0, 0, 0),
            LogEvent::ProcessorVirtualized(x) => (11, x as _, 0, 0, 0),
            LogEvent::FailedToMap => (12, 0, 0, 0, 0),
            LogEvent::DelayedStart => (13, 0, 0, 0, 0),
            LogEvent::HxPosedInit(x, y) => (14, x, y, 0, 0),
            LogEvent::NtVersion(x) => (15, x, 0, 0, 0),
            LogEvent::WindowsVersion(x, y) => (16, x as _, y as _, 0, 0),
            LogEvent::FailedToAllocate => (17, 0, 0, 0, 0),
            LogEvent::Exception(x) => (18, x as _, 0, 0, 0),
            LogEvent::Catastrophic(x, y, z) => (19, x, y as _, z as _, 0),
            LogEvent::ProcessorReady(x, y) => (20, x as _, y, 0, 0),
            LogEvent::LaunchingProcessor => (21, 0, 0, 0, 0),
            LogEvent::Vmclear(x, y) => (22, x, y, 0, 0),
            LogEvent::Vmptrld(x, y) => (23, x, y, 0, 0),
            LogEvent::Vmxon(x, y) => (24, x, y, 0, 0),
            LogEvent::Panic(x, y) => (25, x, y, 0, 0),
            LogEvent::WritingAsyncBuffer(x, y) => (26, x, y, 0, 0),
            LogEvent::WrittenAsyncBuffer(x, y) => (27, x, y, 0, 0),
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
