use bitfield_struct::bitfield;

#[bitfield(u64)]
pub struct InterProcessorInterrupt {
    pub vector: u8,
    #[bits(3)]
    pub delivery_mode: DeliveryMode,
    #[bits(1)]
    pub destination_mode: DestinationMode,
    #[bits(2)]
    reserved: u64,
    #[bits(1)]
    pub level: Level,
    #[bits(1)]
    pub trigger_mode: TriggerMode,
    #[bits(2)]
    reserved2: u64,
    #[bits(2)]
    pub destination: DestinationShorthand,
    #[bits(12)]
    reserved3: u64,
    pub apic_id: u32
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DeliveryMode {
    /// Just delivers
    Fixed = 0,
    /// Same as fixed. But discouraged in use.
    Lowest = 1,
    /// Delivers an SMI. Vector must be 0.
    SystemManagement = 0b_10,
    /// Delivers an NMI. Vector is ignored.
    NonMaskable = 0b_100,
    /// Delivers an INIT request to all targets. Vector must be 0.
    INIT = 0b_101,
    /// Delivers a special [`DeliveryMode::INIT`] to targets.
    StartUp = 0b_110,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Destination {
    Physical(u8),
    Logical(DestinationShorthand),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DestinationMode {
    /// If chosen, [`DestinationShorthand`] is ignored and target APIC id of the processor is used.
    Physical = 0,
    /// If chosen, [`DestinationShorthand`] is utilized
    Logical = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DestinationShorthand {
    None = 0,
    SelfOnly = 0b_1,
    AllIncludingSelf = 0b_10,
    AllExcludingSelf = 0b_11,
}

impl DestinationShorthand {
    pub const fn into_bits(self) -> u16 {
        self as _
    }
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => DestinationShorthand::None,
            1 => DestinationShorthand::SelfOnly,
            0b_10 => DestinationShorthand::AllIncludingSelf,
            0b_11 => DestinationShorthand::AllExcludingSelf,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// Ignored for all delivery modes except IPI
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

impl TriggerMode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => TriggerMode::Edge,
            1 => TriggerMode::Level,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// Not much is said in Intel SDM. But must be 0 for INIT.
pub enum Level {
    Assert = 0,
    DeAssert = 1,
}

impl Level {
    pub const fn into_bits(self) -> u16 {
        self as _
    }
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => Level::Assert,
            1 => Level::DeAssert,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
/// This is a read only field
pub enum DeliveryStatus {
    /// Indicates this local APIC has completed sending previous IPIs
    Idle = 0,
    /// Indicates this local APIC is still waiting to deliver the last IPI
    Pending = 1,
}

impl DeliveryStatus {
    pub const fn into_bits(self) -> u16 {
        self as _
    }
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => DeliveryStatus::Idle,
            1 => DeliveryStatus::Pending,
            _ => unreachable!(),
        }
    }
}

impl DestinationMode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }
    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => DestinationMode::Physical,
            1 => DestinationMode::Logical,
            _ => unreachable!(),
        }
    }
}

impl DeliveryMode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => DeliveryMode::Fixed,
            1 => DeliveryMode::Lowest,
            0b_10 => DeliveryMode::SystemManagement,
            0b_100 => DeliveryMode::NonMaskable,
            0b_101 => DeliveryMode::INIT,
            0b_110 => DeliveryMode::StartUp,
            _ => unreachable!(),
        }
    }
}