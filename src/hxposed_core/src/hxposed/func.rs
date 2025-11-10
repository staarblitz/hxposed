#[derive(Debug)]
pub enum ServiceFunction {
    Authorize,
    GetState,
    Unknown,
}

impl ServiceFunction {
    pub const fn into_bits(self) -> u16 {
        self as _
    }

    pub const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::Authorize,
            1 => Self::GetState,
            _ => Self::Unknown,
        }
    }
}
