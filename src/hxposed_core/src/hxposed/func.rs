#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ServiceFunction {
    // Separated by CATEGORY - FUNCTION
    GetState = 0b_0000_0000,

    OpenProcess = 0b_0001_0000,
    CloseProcess = 0b_0001_0001,
    GetProcessField = 0b_0001_0010,
    SetProcessField = 0b_0001_0011,

    RegisterNotifyEvent = 0b_0010_0000,
    UnregisterNotifyEvent = 0b_0010_0001,

    AllocateMemory = 0b_0011_0000,
    FreeMemory = 0b_0011_0001,
    GetSetPageAttribute = 0b_0011_0010,
    MapVaToPa = 0b_0011_0011,
    TranslateAddress = 0b_0011_0100,

    OpenThread = 0b_0100_0000,
    CloseThread = 0b_0100_0001,
    GetThreadField = 0b_0100_0010,
    SetThreadField = 0b_0100_0011,

    OpenToken = 0b_0101_0000,
    CloseToken = 0b_0101_0001,
    GetTokenField = 0b_0101_0011,
    SetTokenField = 0b_0101_0100,

    MsrIo = 0b_0110_0000,
    ExecutePrivilegedInstruction = 0b_0110_0001,
    InterProcessorInterrupt = 0b_0110_0010
}

impl ServiceFunction {
    pub const fn into_bits(self) -> u16 {
        self as u16
    }

    pub const fn from_bits(bits: u8) -> Self {
        // get rekt
        unsafe {
            core::mem::transmute(bits)
        }
    }
}