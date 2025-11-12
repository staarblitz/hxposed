#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ErrorCode {
    Unknown = 0,
    Ok = 1,
    NotAllowed = 2,
    NotLoaded = 3
}

impl ErrorCode {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::Unknown,
            1 => Self::Ok,
            2 => Self::NotAllowed,
            3 => Self::NotLoaded,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum ErrorSource {
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
