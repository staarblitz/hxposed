#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum NotAllowedReason {
    Unknown = 0,
    LockHeld = 2,
    PageNotPresent = 3,
    MappingsExist = 4,
    AccessViolation = 5,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum NotFoundReason {
    Unknown = 0,
    Process = 1,
    Mdl = 3,
    Thread = 4,
    ServiceFunction = 5,
    Token = 6,
    Callback = 7,
    Event = 9,
    Field = 10,
    Handle = 11,
}

impl NotFoundReason {
    pub const fn into_bits(self) -> u32 {
        self as _
    }

    pub const fn from_bits(value: u32) -> Self {
        match value {
            1 => Self::Process,
            3 => Self::Mdl,
            4 => Self::Thread,
            5 => Self::ServiceFunction,
            6 => Self::Token,
            7 => Self::Callback,
            9 => Self::Event,
            10 => Self::Field,
            11 => Self::Handle,
            _ => Self::Unknown
        }
    }
}

impl NotAllowedReason {
    pub const fn into_bits(self) -> u32 {
        self as _
    }

    pub const fn from_bits(value: u32) -> Self {
        match value {
            2 => Self::LockHeld,
            3 => Self::PageNotPresent,
            4 => Self::MappingsExist,
            5 => Self::AccessViolation,
            _ => Self::Unknown,
        }
    }
}