use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug, Clone, Copy)]
    #[repr(transparent)]
    pub struct MemoryProtection : u32 {
        const NONE = 0;
        const NO_ACCESS = 0x1;
        const READONLY = 0x2;
        const READWRITE = 0x4;
        const WRITECOPY = 0x8;
        const EXECUTE  = 0x10;
        const EXECUTE_READ = 0x20;
        const EXECUTE_READWRITE = 0x40;
        const EXECUTE_WRITECOPY = 0x80;
        const GUARD = 0x100;
        const NO_CACHE = 0x200;
        const WRITE_COMBINE = 0x400;
        const INVALID = 0x40000000;
    }
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
