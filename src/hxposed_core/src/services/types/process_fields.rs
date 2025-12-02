use bitfield_struct::bitfield;

#[bitfield(u8)]
pub struct ProcessProtection {
    #[bits(3)]
    pub protection_type: ProtectionType,
    #[bits(1)]
    pub audit: bool,
    #[bits(4)]
    pub signer: ProtectionSigner,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProtectionSigner {
    #[default]
    None = 0,
    Authenticode = 1,
    CodeGen = 2,
    AntiMalware = 3,
    Lsa = 4,
    Windows = 5,
    WinTcb = 6,
    Max = 7,
}

impl ProtectionSigner {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProtectionSigner::Authenticode,
            2 => ProtectionSigner::CodeGen,
            3 => ProtectionSigner::AntiMalware,
            4 => ProtectionSigner::Lsa,
            5 => ProtectionSigner::Windows,
            6 => ProtectionSigner::WinTcb,
            7 => ProtectionSigner::Max,
            _ => ProtectionSigner::None
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

#[bitfield(u16)]
pub struct ProcessSignatureLevels {
    #[bits(8)]
    pub signature_level: ProcessSignatureLevel,
    #[bits(8)]
    pub section_signature_level: u8
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProcessSignatureLevel {
    #[default]
    Unchecked = 0,
    Unsigned = 1,
    Enterprise = 2,
    Custom = 3,
    Authenticode = 4,
    Custom2 = 5,
    Store = 6,
    AntiMalware = 7,
    Microsoft = 8,
    Custom4 = 9,
    Custom5 = 10,
    DynamicCodeGen = 11,
    Windows = 12,
    WindowsPPL = 13,
    WindowsTcb = 14,
    Custom6 = 15
}

impl ProcessSignatureLevel {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProcessSignatureLevel::Unsigned,
            2 => ProcessSignatureLevel::Enterprise,
            3 => ProcessSignatureLevel::Custom,
            4 => ProcessSignatureLevel::Authenticode,
            5 => ProcessSignatureLevel::Custom2,
            6 => ProcessSignatureLevel::Store,
            7 => ProcessSignatureLevel::AntiMalware,
            8 => ProcessSignatureLevel::Microsoft,
            9 => ProcessSignatureLevel::Custom4,
            10 => ProcessSignatureLevel::Custom5,
            11 => ProcessSignatureLevel::DynamicCodeGen,
            12 => ProcessSignatureLevel::Windows,
            13 => ProcessSignatureLevel::WindowsPPL,
            14 => ProcessSignatureLevel::WindowsTcb,
            15 => ProcessSignatureLevel::Custom6,
            _ => ProcessSignatureLevel::Unchecked,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(u8)]
pub enum ProtectionType {
    #[default]
    None = 0,
    Light = 1,
    Protected = 2,
    Max = 3,
}

impl ProtectionType {
    pub const fn from_bits(value: u8) -> Self {
        match value {
            1 => ProtectionType::Light,
            2 => ProtectionType::Protected,
            3 => ProtectionType::Max,
            _ => ProtectionType::None,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as _
    }
}