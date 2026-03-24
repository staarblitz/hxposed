use bitfield_struct::bitfield;

pub const GENERAL_PROTECTION_FAULT: VmInterruptInfo =
    VmInterruptInfo::new()
        .with_interrupt_type(InterruptionType::HardwareException)
        .with_vector(ExceptionVector::GeneralProtectionFault)
        .with_valid(true)
        .with_deliver_error_code(true);

pub const UNKNOWN_OPCODE: VmInterruptInfo =
    VmInterruptInfo::new()
        .with_interrupt_type(InterruptionType::HardwareException)
        .with_vector(ExceptionVector::InvalidOpcode)
        .with_valid(true);

#[bitfield(u32)]
pub struct VmInterruptInfo {
    #[bits(8)]
    pub vector: ExceptionVector,
    #[bits(3)]
    pub interrupt_type: InterruptionType,
    pub deliver_error_code: bool,
    #[bits(19)]
    reserved: u64,
    pub valid: bool,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExceptionVector {
    DivisionError = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    SegmentOverrun = 9,
    InvalidTSS = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    FloatingPointError = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SIMD = 19,
    VirtualizationException = 20,
    ControlProtectionException = 21,
    MaskableInterrupts = 32,
    Unknown = u8::MAX,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InterruptionType {
    External = 0,
    Reserved = 1,
    NMI = 2,
    HardwareException = 3,
    SoftwareInterrupt = 4,
    Privileged = 5,
    SoftwareException = 6,
    Other = 7,
}

impl InterruptionType {
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub const fn from_bits(bits: u8) -> Self {
        if bits >= 0 && bits < 8 {
            unsafe {
                // im sick of writing switch cases
                core::mem::transmute(bits)
            }
        } else {
            Self::Other
        }
    }
}

impl ExceptionVector {
    pub const fn into_bits(self) -> u8 {
        self as u8
    }

    pub const fn from_bits(bits: u8) -> Self {
        if bits >= 0 && bits < 8 {
            unsafe {
                // im sick of writing switch cases
                core::mem::transmute(bits)
            }
        } else {
            Self::Unknown
        }
    }
}
