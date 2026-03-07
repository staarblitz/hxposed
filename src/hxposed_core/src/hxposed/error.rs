#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum NotAllowedReason {
    MissingPermissions = 1,
    LockHeld = 2,
    PageNotPresent = 3,
    MappingsExist = 4,
    AccessViolation = 5,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum NotFoundReason {
    Process = 1,
    Plugin = 2,
    Mdl = 3,
    Thread = 4,
    ServiceFunction = 5,
    Token = 6,
    Callback = 7,
    HxInfo = 8,
    Event = 9,
    Field = 10,
}

impl NotFoundReason {
    pub const fn into_bits(self) -> u32 {
        self as _
    }

    pub const fn from_bits(value: u32) -> Self {
        match value {
            1 => Self::Process,
            2 => Self::Plugin,
            3 => Self::Mdl,
            4 => Self::Thread,
            5 => Self::ServiceFunction,
            6 => Self::Token,
            7 => Self::Callback,
            8 => Self::HxInfo,
            9 => Self::Event,
            10 => Self::Field,
            _ => unreachable!(),
        }
    }
}

impl NotAllowedReason {
    pub const fn into_bits(self) -> u32 {
        self as _
    }

    pub const fn from_bits(value: u32) -> Self {
        match value {
            1 => Self::MissingPermissions,
            2 => Self::LockHeld,
            3 => Self::PageNotPresent,
            4 => Self::MappingsExist,
            5 => Self::AccessViolation,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[repr(u32)]
pub enum ErrorSource {
    #[default]
    Nt = 0,
    Hv = 1,
    Hx = 2,
}

impl ErrorSource {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::Nt,
            1 => Self::Hv,
            2 => Self::Hx,
            _ => unreachable!(),
        }
    }
}
