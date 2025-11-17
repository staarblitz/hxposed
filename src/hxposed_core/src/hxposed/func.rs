#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ServiceFunction {
    #[default]
    Unknown = 0,
    Authorize = 1,
    GetState = 2,
    OpenProcess = 3,
    CloseProcess = 4,
}

impl ServiceFunction {
    pub const fn into_bits(self) -> u16 {
        self as u16
    }

    pub const fn from_bits(bits: u16) -> Self {
        match bits {
            0 => ServiceFunction::Unknown,
            1 => ServiceFunction::Authorize,
            2 => ServiceFunction::GetState,
            3 => ServiceFunction::OpenProcess,
            4 => ServiceFunction::CloseProcess,
            _ => ServiceFunction::Unknown
        }
    }
}