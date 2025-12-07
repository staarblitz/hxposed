#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[repr(u32)]
pub enum InternalErrorCode {
    #[default]
    Unknown = 0,
    Ok = 1,
    /// See [NotAllowedReason]. Put on arg1
    NotAllowed = 2,
    NotLoaded = 3,
    NotFound = 4,
    InvalidParams = 5
}

impl InternalErrorCode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Ok,
            2 => Self::NotAllowed,
            3 => Self::NotLoaded,
            4 => Self::NotFound,
            5 => Self::InvalidParams,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[repr(u32)]
pub enum NotAllowedReason {
    #[default]
    Unknown = 0,
    PluginNotLoaded = u32::MAX,
    MissingPermissions = 1
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
#[repr(u32)]
pub enum NotFoundReason {
    #[default]
    Unknown = 0,
    Process = 1,
    Plugin = 2,
    Mdl = 3,
    Thread = 4
}

impl NotFoundReason {
    pub const fn into_bits(self) -> u32 {self as _}

    pub const fn from_bits(value: u32) -> Self {
        match value {
            1 => Self::Process,
            2 => Self::Plugin,
            3 => Self::Mdl,
            4 => Self::Thread,
            _ => Self::Unknown
        }
    }
}

impl NotAllowedReason {
    pub const fn into_bits(self) -> u32 {self as _}

    pub const fn from_bits(value: u32) -> Self {
        match value {
            u32::MAX => Self::PluginNotLoaded,
            1 => Self::MissingPermissions,
            _ => Self::Unknown
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
            _ => unreachable!()
        }
    }
}
