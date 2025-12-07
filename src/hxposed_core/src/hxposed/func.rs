#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum ServiceFunction {
    #[default]
    Unknown = 0,
    Authorize = 1,
    GetState = 2,
    OpenProcess = 3,
    CloseProcess = 4,
    KillProcess = 5,
    AddAsyncHandler = 6,
    RemoveAsyncHandler = 7,
    GetProcessField = 8,
    SetProcessField = 9,
    ProcessVMOperation = 10,
    ProtectProcessMemory = 11,
    AllocateMemory = 12,
    MapMemory = 13,
    FreeMemory = 14,
    GetProcessThreads = 15,
    OpenThread = 16,
    CloseThread = 17,
    SuspendResumeThread = 18,
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
            5 => ServiceFunction::KillProcess,
            6 => ServiceFunction::AddAsyncHandler,
            7 => ServiceFunction::RemoveAsyncHandler,
            8 => ServiceFunction::GetProcessField,
            9 => ServiceFunction::SetProcessField,
            10 => ServiceFunction::ProcessVMOperation,
            11 => ServiceFunction::ProtectProcessMemory,
            12 => ServiceFunction::AllocateMemory,
            13 => ServiceFunction::MapMemory,
            14 => ServiceFunction::FreeMemory,
            15 => ServiceFunction::GetProcessThreads,
            16 => ServiceFunction::OpenThread,
            17 => ServiceFunction::CloseThread,
            18 => ServiceFunction::SuspendResumeThread,
            _ => ServiceFunction::Unknown
        }
    }
}