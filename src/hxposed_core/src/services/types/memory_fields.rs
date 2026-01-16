use bitflag::bitflag;

#[bitflag(u32)]
#[derive(Clone, Copy, Debug, Default)]
pub enum MemoryProtection {
    None = 0,
    NoAccess = 0x1,
    ReadOnly = 0x2,
    ReadWrite = 0x4,
    WriteCopy = 0x8,
    Execute = 0x10,
    ExecuteRead = 0x20,
    ExecuteReadWrite = 0x40,
    ExecuteWriteCopy = 0x80,
    Guard = 0x100,
    NoCache = 0x200,
    WriteCombine = 0x400,
    Invalid = 0x40000000
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryType {
    Virtual,
    Physical,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum MemoryPool {
    #[default]
    NonPaged,
}

impl MemoryPool {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            _ => Self::NonPaged,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum KernelMemoryState {
    None,
    Allocated,
    Mapped(usize),
    Freed,
}
