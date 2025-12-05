#[derive(Copy, Clone, Default, Debug)]
#[repr(u32)]
pub enum MemoryProtection {
    #[default]
    NoAccess = 0x1,
    ReadOnly = 0x2,
    ReadWrite = 0x4,
    WriteCopy = 0x8,
    Execute = 0x10,
    ExecuteRead = 0x20,
    ExecuteReadWrite = 0x40,
    Invalid = 0x40000000,
}