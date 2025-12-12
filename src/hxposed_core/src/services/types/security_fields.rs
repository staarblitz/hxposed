use alloc::string::String;
use bitflags::bitflags;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct Luid {
    pub low: u32,
    pub high: i32,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

bitflags! {
        #[derive(Clone)]
    pub struct TokenFlags: u32 {
        const TOKEN_FLAGS_NONE = 0;
        const TOKEN_HAS_TRAVERSE_PRIVILEGE = 0x00000001;
        const TOKEN_HAS_BACKUP_PRIVILEGE = 0x0002;
        const TOKEN_HAS_RESTORE_PRIVILEGE= 0x0004;
        const TOKEN_WRITE_RESTRICTED= 0x0008;
        const TOKEN_HAS_ADMIN_GROUP= 0x0008;
        const TOKEN_IS_RESTRICTED=0x0010;
        const TOKEN_SESSION_NOT_REFERENCED=0x0020;
        const TOKEN_SANDBOX_INERT=0x0040;
        const TOKEN_HAS_IMPERSONATE_PRIVILEGE=0x0080;
        const SE_BACKUP_PRIVILEGES_CHECKED=0x0100;
        const TOKEN_VIRTUALIZE_ALLOWED=0x0200;
        const TOKEN_VIRTUALIZE_ENABLED=      0x0400;
        const TOKEN_IS_FILTERED=0x0800;
        const TOKEN_UIACCESS=0x1000;
        const TOKEN_NOT_LOW=0x2000;
    }
}


// why cargo fmt doesn't work?
bitflags! {
    #[derive(Debug, Default, Clone)]
    pub struct TokenPrivilege: u64 {
const SeCreateTokenPrivilege                    = 1 << 2;
const SeAssignPrimaryTokenPrivilege             = 1 << 3;
const SeLockMemoryPrivilege                     = 1 << 4;
const SeIncreaseQuotaPrivilege                  = 1 << 5;
const SeMachineAccountPrivilege                 = 1 << 6;
const SeTcbPrivilege                            = 1 << 7;
const SeSecurityPrivilege                       = 1 << 8;
const SeTakeOwnershipPrivilege                  = 1 << 9;
const SeLoadDriverPrivilege                     = 1 << 10;
const SeSystemProfilePrivilege                  = 1 << 11;
const SeSystemtimePrivilege                     = 1 << 12;
const SeProfileSingleProcessPrivilege           = 1 << 13;
const SeIncreaseBasePriorityPrivilege           = 1 << 14;
const SeCreatePagefilePrivilege                 = 1 << 15;
const SeCreatePermanentPrivilege                = 1 << 16;
const SeBackupPrivilege                         = 1 << 17;
const SeRestorePrivilege                        = 1 << 18;
const SeShutdownPrivilege                       = 1 << 19;
const SeDebugPrivilege                          = 1 << 20;
const SeAuditPrivilege                          = 1 << 21;
const SeSystemEnvironmentPrivilege              = 1 << 22;
const SeChangeNotifyPrivilege                   = 1 << 23;
const SeRemoteShutdownPrivilege                 = 1 << 24;
const SeUndockPrivilege                         = 1 << 25;
const SeSyncAgentPrivilege                      = 1 << 26;
const SeEnableDelegationPrivilege               = 1 << 27;
const SeManageVolumePrivilege                   = 1 << 28;
const SeImpersonatePrivilege                    = 1 << 29;
const SeCreateGlobalPrivilege                   = 1 << 30;
const SeTrustedCredManAccessPrivilege           = 1 << 31;
const SeRelabelPrivilege                        = 1 << 32;
const SeIncreaseWorkingSetPrivilege             = 1 << 33;
const SeTimeZonePrivilege                       = 1 << 34;
const SeCreateSymbolicLinkPrivilege             = 1 << 35;
const SeDelegateSessionUserImpersonatePrivilege = 1 << 36;

    }
}
