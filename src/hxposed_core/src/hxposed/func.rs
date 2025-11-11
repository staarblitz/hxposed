#[derive(Debug)]
pub enum ServiceFunction {
    Authorize,
    GetState,
    Unknown,
    OpenProcess
}

impl ServiceFunction {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::Authorize,
            1 => Self::GetState,
            3 => Self::OpenProcess,
            _ => Self::Unknown,
        }
    }
}
