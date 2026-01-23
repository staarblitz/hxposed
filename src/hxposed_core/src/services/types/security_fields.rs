#![allow(non_upper_case_globals)]

use alloc::string::String;
use bitflag::bitflag;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Luid {
    pub low: u32,
    pub high: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub enum TokenType {
    Primary = 0,
    Impersonation = 1,
}

impl TokenType {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Primary,
            1 => Self::Impersonation,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub enum ImpersonationLevel {
    Anonymous = 0,
    Identification = 1,
    Impersonation = 2,
    Delegation = 3,
}

impl ImpersonationLevel {
    pub const fn into_bits(self) -> u8 {
        self as _
    }

    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Anonymous,
            1 => Self::Identification,
            2 => Self::Impersonation,
            3 => Self::Delegation,
            _ => unreachable!(),
        }
    }
}

pub struct TokenSource {
    pub name: String,
    pub luid: Luid,
}



#[bitflag(u32)]
#[derive(Debug, Copy, Clone, Default)]
pub enum TokenFlags {
    #[default]
    None = 0,
    HasTraversePrivilege = 0x00000001,
    HasBackupPrivilege = 0x0002,
    HasRestorePrivilege = 0x0004,
    WriteRestricted = 0x0008,
    HasAdminGroup = 0x0008,
    IsRestricted = 0x0010,
    SessionNotReferenced = 0x0020,
    SandboxInert = 0x0040,
    HasImpersonatePrivilege = 0x0080,
    SeBackupPrivilegesChecked = 0x0100,
    VirtualizeAllowed = 0x0200,
    VirtualizeEnabled = 0x0400,
    IsFiltered = 0x0800,
    Uiaccess = 0x1000,
    NotLow = 0x2000,
}


// why cargo fmt doesn't work?
#[bitflag(u64)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum TokenPrivilege {
    #[default]
    None = 0,
    SeCreateTokenPrivilege                    = 1 << 2,
    SeAssignPrimaryTokenPrivilege             = 1 << 3,
    SeLockMemoryPrivilege                     = 1 << 4,
    SeIncreaseQuotaPrivilege                  = 1 << 5,
    SeMachineAccountPrivilege                 = 1 << 6,
    SeTcbPrivilege                            = 1 << 7,
    SeSecurityPrivilege                       = 1 << 8,
    SeTakeOwnershipPrivilege                  = 1 << 9,
    SeLoadDriverPrivilege                     = 1 << 10,
    SeSystemProfilePrivilege                  = 1 << 11,
    SeSystemTimePrivilege                     = 1 << 12,
    SeProfileSingleProcessPrivilege           = 1 << 13,
    SeIncreaseBasePriorityPrivilege           = 1 << 14,
    SeCreatePagefilePrivilege                 = 1 << 15,
    SeCreatePermanentPrivilege                = 1 << 16,
    SeBackupPrivilege                         = 1 << 17,
    SeRestorePrivilege                        = 1 << 18,
    SeShutdownPrivilege                       = 1 << 19,
    SeDebugPrivilege                          = 1 << 20,
    SeAuditPrivilege                          = 1 << 21,
    SeSystemEnvironmentPrivilege              = 1 << 22,
    SeChangeNotifyPrivilege                   = 1 << 23,
    SeRemoteShutdownPrivilege                 = 1 << 24,
    SeUndockPrivilege                         = 1 << 25,
    SeSyncAgentPrivilege                      = 1 << 26,
    SeEnableDelegationPrivilege               = 1 << 27,
    SeManageVolumePrivilege                   = 1 << 28,
    SeImpersonatePrivilege                    = 1 << 29,
    SeCreateGlobalPrivilege                   = 1 << 30,
    SeTrustedCredManAccessPrivilege           = 1 << 31,
    SeRelabelPrivilege                        = 1 << 32,
    SeIncreaseWorkingSetPrivilege             = 1 << 33,
    SeTimeZonePrivilege                       = 1 << 34,
    SeCreateSymbolicLinkPrivilege             = 1 << 35,
    SeDelegateSessionUserImpersonatePrivilege = 1 << 36,
}