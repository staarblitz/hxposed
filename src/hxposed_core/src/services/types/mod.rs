pub mod process_fields {
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
}