use alloc::string::String;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Luid {
    pub low: u32,
    pub high: i32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum TokenType {
    Primary = 0,
    Impersonation = 1
}

impl TokenType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Primary,
            1 => Self::Impersonation,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum ImpersonationLevel {
    Anonymous = 0,
    Identification = 1,
    Impersonation = 2,
    Delegation = 3
}

impl ImpersonationLevel {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Anonymous,
            1 => Self::Identification,
            2 => Self::Impersonation,
            3 => Self::Delegation,
            _ => unreachable!(),
        }
    }
}

pub struct TokenSource {
    pub name: String,
    pub luid: Luid,
}