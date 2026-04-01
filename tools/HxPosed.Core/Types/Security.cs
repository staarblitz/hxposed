using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Types
{
    public enum TokenType
    {
        Primary = 0,
        Impersonation = 1,
    }

    public enum TokenImpersonationLevel
    {
        Anonymous = 0,
        Identification = 1,
        Impersonation = 2,
        Delegation = 3,
    }

    [StructLayout(LayoutKind.Explicit)]
    public partial struct TokenPrivileges
    {
        [FieldOffset(0)]
        public ulong All;

        [FieldOffset(0)]
        public _Anonymous_e__Union Anonymous;

        public ulong RESERVED
        {
            readonly get
            {
                return Anonymous.RESERVED;
            }

            set
            {
                Anonymous.RESERVED = value;
            }
        }

        public ulong SeCreateTokenPrivilege
        {
            readonly get
            {
                return Anonymous.SeCreateTokenPrivilege;
            }

            set
            {
                Anonymous.SeCreateTokenPrivilege = value;
            }
        }

        public ulong SeAssignPrimaryTokenPrivilege
        {
            readonly get
            {
                return Anonymous.SeAssignPrimaryTokenPrivilege;
            }

            set
            {
                Anonymous.SeAssignPrimaryTokenPrivilege = value;
            }
        }

        public ulong SeLockMemoryPrivilege
        {
            readonly get
            {
                return Anonymous.SeLockMemoryPrivilege;
            }

            set
            {
                Anonymous.SeLockMemoryPrivilege = value;
            }
        }

        public ulong SeIncreaseQuotaPrivilege
        {
            readonly get
            {
                return Anonymous.SeIncreaseQuotaPrivilege;
            }

            set
            {
                Anonymous.SeIncreaseQuotaPrivilege = value;
            }
        }

        public ulong SeMachineAccountPrivilege
        {
            readonly get
            {
                return Anonymous.SeMachineAccountPrivilege;
            }

            set
            {
                Anonymous.SeMachineAccountPrivilege = value;
            }
        }

        public ulong SeTcbPrivilege
        {
            readonly get
            {
                return Anonymous.SeTcbPrivilege;
            }

            set
            {
                Anonymous.SeTcbPrivilege = value;
            }
        }

        public ulong SeSecurityPrivilege
        {
            readonly get
            {
                return Anonymous.SeSecurityPrivilege;
            }

            set
            {
                Anonymous.SeSecurityPrivilege = value;
            }
        }

        public ulong SeTakeOwnershipPrivilege
        {
            readonly get
            {
                return Anonymous.SeTakeOwnershipPrivilege;
            }

            set
            {
                Anonymous.SeTakeOwnershipPrivilege = value;
            }
        }

        public ulong SeLoadDriverPrivilege
        {
            readonly get
            {
                return Anonymous.SeLoadDriverPrivilege;
            }

            set
            {
                Anonymous.SeLoadDriverPrivilege = value;
            }
        }

        public ulong SeSystemProfilePrivilege
        {
            readonly get
            {
                return Anonymous.SeSystemProfilePrivilege;
            }

            set
            {
                Anonymous.SeSystemProfilePrivilege = value;
            }
        }

        public ulong SeSystemtimePrivilege
        {
            readonly get
            {
                return Anonymous.SeSystemtimePrivilege;
            }

            set
            {
                Anonymous.SeSystemtimePrivilege = value;
            }
        }

        public ulong SeProfileSingleProcessPrivilege
        {
            readonly get
            {
                return Anonymous.SeProfileSingleProcessPrivilege;
            }

            set
            {
                Anonymous.SeProfileSingleProcessPrivilege = value;
            }
        }

        public ulong SeIncreaseBasePriorityPrivilege
        {
            readonly get
            {
                return Anonymous.SeIncreaseBasePriorityPrivilege;
            }

            set
            {
                Anonymous.SeIncreaseBasePriorityPrivilege = value;
            }
        }

        public ulong SeCreatePagefilePrivilege
        {
            readonly get
            {
                return Anonymous.SeCreatePagefilePrivilege;
            }

            set
            {
                Anonymous.SeCreatePagefilePrivilege = value;
            }
        }

        public ulong SeCreatePermanentPrivilege
        {
            readonly get
            {
                return Anonymous.SeCreatePermanentPrivilege;
            }

            set
            {
                Anonymous.SeCreatePermanentPrivilege = value;
            }
        }

        public ulong SeBackupPrivilege
        {
            readonly get
            {
                return Anonymous.SeBackupPrivilege;
            }

            set
            {
                Anonymous.SeBackupPrivilege = value;
            }
        }

        public ulong SeRestorePrivilege
        {
            readonly get
            {
                return Anonymous.SeRestorePrivilege;
            }

            set
            {
                Anonymous.SeRestorePrivilege = value;
            }
        }

        public ulong SeShutdownPrivilege
        {
            readonly get
            {
                return Anonymous.SeShutdownPrivilege;
            }

            set
            {
                Anonymous.SeShutdownPrivilege = value;
            }
        }

        public ulong SeDebugPrivilege
        {
            readonly get
            {
                return Anonymous.SeDebugPrivilege;
            }

            set
            {
                Anonymous.SeDebugPrivilege = value;
            }
        }

        public ulong SeAuditPrivilege
        {
            readonly get
            {
                return Anonymous.SeAuditPrivilege;
            }

            set
            {
                Anonymous.SeAuditPrivilege = value;
            }
        }

        public ulong SeSystemEnvironmentPrivilege
        {
            readonly get
            {
                return Anonymous.SeSystemEnvironmentPrivilege;
            }

            set
            {
                Anonymous.SeSystemEnvironmentPrivilege = value;
            }
        }

        public ulong SeChangeNotifyPrivilege
        {
            readonly get
            {
                return Anonymous.SeChangeNotifyPrivilege;
            }

            set
            {
                Anonymous.SeChangeNotifyPrivilege = value;
            }
        }

        public ulong SeRemoteShutdownPrivilege
        {
            readonly get
            {
                return Anonymous.SeRemoteShutdownPrivilege;
            }

            set
            {
                Anonymous.SeRemoteShutdownPrivilege = value;
            }
        }

        public ulong SeUndockPrivilege
        {
            readonly get
            {
                return Anonymous.SeUndockPrivilege;
            }

            set
            {
                Anonymous.SeUndockPrivilege = value;
            }
        }

        public ulong SeSyncAgentPrivilege
        {
            readonly get
            {
                return Anonymous.SeSyncAgentPrivilege;
            }

            set
            {
                Anonymous.SeSyncAgentPrivilege = value;
            }
        }

        public ulong SeEnableDelegationPrivilege
        {
            readonly get
            {
                return Anonymous.SeEnableDelegationPrivilege;
            }

            set
            {
                Anonymous.SeEnableDelegationPrivilege = value;
            }
        }

        public ulong SeManageVolumePrivilege
        {
            readonly get
            {
                return Anonymous.SeManageVolumePrivilege;
            }

            set
            {
                Anonymous.SeManageVolumePrivilege = value;
            }
        }

        public ulong SeImpersonatePrivilege
        {
            readonly get
            {
                return Anonymous.SeImpersonatePrivilege;
            }

            set
            {
                Anonymous.SeImpersonatePrivilege = value;
            }
        }

        public ulong SeCreateGlobalPrivilege
        {
            readonly get
            {
                return Anonymous.SeCreateGlobalPrivilege;
            }

            set
            {
                Anonymous.SeCreateGlobalPrivilege = value;
            }
        }

        public ulong SeTrustedCredManAccessPrivilege
        {
            readonly get
            {
                return Anonymous.SeTrustedCredManAccessPrivilege;
            }

            set
            {
                Anonymous.SeTrustedCredManAccessPrivilege = value;
            }
        }

        public ulong SeRelabelPrivilege
        {
            readonly get
            {
                return Anonymous.SeRelabelPrivilege;
            }

            set
            {
                Anonymous.SeRelabelPrivilege = value;
            }
        }

        public ulong SeIncreaseWorkingSetPrivilege
        {
            readonly get
            {
                return Anonymous.SeIncreaseWorkingSetPrivilege;
            }

            set
            {
                Anonymous.SeIncreaseWorkingSetPrivilege = value;
            }
        }

        public ulong SeTimeZonePrivilege
        {
            readonly get
            {
                return Anonymous.SeTimeZonePrivilege;
            }

            set
            {
                Anonymous.SeTimeZonePrivilege = value;
            }
        }

        public ulong SeCreateSymbolicLinkPrivilege
        {
            readonly get
            {
                return Anonymous.SeCreateSymbolicLinkPrivilege;
            }

            set
            {
                Anonymous.SeCreateSymbolicLinkPrivilege = value;
            }
        }

        public ulong SeDelegateSessionUserImpersonatePrivilege
        {
            readonly get
            {
                return Anonymous.SeDelegateSessionUserImpersonatePrivilege;
            }

            set
            {
                Anonymous.SeDelegateSessionUserImpersonatePrivilege = value;
            }
        }

        public ulong RESERVED2
        {
            readonly get
            {
                return Anonymous.RESERVED2;
            }

            set
            {
                Anonymous.RESERVED2 = value;
            }
        }

        [StructLayout(LayoutKind.Explicit, Pack = 1)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            public ulong _bitfield;
            public ulong RESERVED
            {
                readonly get
                {
                    return _bitfield & 0x3UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~0x3UL) | (value & 0x3UL);
                }
            }

            
            public ulong SeCreateTokenPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 2) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 2)) | ((value & 0x1UL) << 2);
                }
            }

            
            public ulong SeAssignPrimaryTokenPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 3) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 3)) | ((value & 0x1UL) << 3);
                }
            }

            
            public ulong SeLockMemoryPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 4) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 4)) | ((value & 0x1UL) << 4);
                }
            }

            
            public ulong SeIncreaseQuotaPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 5) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 5)) | ((value & 0x1UL) << 5);
                }
            }

            
            public ulong SeMachineAccountPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 6) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 6)) | ((value & 0x1UL) << 6);
                }
            }

            
            public ulong SeTcbPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 7) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 7)) | ((value & 0x1UL) << 7);
                }
            }

            
            public ulong SeSecurityPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 8) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 8)) | ((value & 0x1UL) << 8);
                }
            }

            
            public ulong SeTakeOwnershipPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 9) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 9)) | ((value & 0x1UL) << 9);
                }
            }

            
            public ulong SeLoadDriverPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 10) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 10)) | ((value & 0x1UL) << 10);
                }
            }

            
            public ulong SeSystemProfilePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 11) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 11)) | ((value & 0x1UL) << 11);
                }
            }

            
            public ulong SeSystemtimePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 12) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 12)) | ((value & 0x1UL) << 12);
                }
            }

            
            public ulong SeProfileSingleProcessPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 13) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 13)) | ((value & 0x1UL) << 13);
                }
            }

            
            public ulong SeIncreaseBasePriorityPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 14) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 14)) | ((value & 0x1UL) << 14);
                }
            }

            
            public ulong SeCreatePagefilePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 15) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 15)) | ((value & 0x1UL) << 15);
                }
            }

            
            public ulong SeCreatePermanentPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 16) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 16)) | ((value & 0x1UL) << 16);
                }
            }

            
            public ulong SeBackupPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 17) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 17)) | ((value & 0x1UL) << 17);
                }
            }

            
            public ulong SeRestorePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 18) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 18)) | ((value & 0x1UL) << 18);
                }
            }

            
            public ulong SeShutdownPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 19) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 19)) | ((value & 0x1UL) << 19);
                }
            }

            
            public ulong SeDebugPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 20) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 20)) | ((value & 0x1UL) << 20);
                }
            }

            
            public ulong SeAuditPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 21) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 21)) | ((value & 0x1UL) << 21);
                }
            }

            
            public ulong SeSystemEnvironmentPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 22) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 22)) | ((value & 0x1UL) << 22);
                }
            }

            
            public ulong SeChangeNotifyPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 23) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 23)) | ((value & 0x1UL) << 23);
                }
            }

            
            public ulong SeRemoteShutdownPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 24) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 24)) | ((value & 0x1UL) << 24);
                }
            }

            
            public ulong SeUndockPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 25) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 25)) | ((value & 0x1UL) << 25);
                }
            }

            
            public ulong SeSyncAgentPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 26) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 26)) | ((value & 0x1UL) << 26);
                }
            }

            
            public ulong SeEnableDelegationPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 27) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 27)) | ((value & 0x1UL) << 27);
                }
            }

            
            public ulong SeManageVolumePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 28) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 28)) | ((value & 0x1UL) << 28);
                }
            }

            
            public ulong SeImpersonatePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 29) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 29)) | ((value & 0x1UL) << 29);
                }
            }

            
            public ulong SeCreateGlobalPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 30) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 30)) | ((value & 0x1UL) << 30);
                }
            }

            
            public ulong SeTrustedCredManAccessPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 31) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 31)) | ((value & 0x1UL) << 31);
                }
            }

            
            public ulong SeRelabelPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 32) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 32)) | ((value & 0x1UL) << 32);
                }
            }

            
            public ulong SeIncreaseWorkingSetPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 33) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 33)) | ((value & 0x1UL) << 33);
                }
            }

            
            public ulong SeTimeZonePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 34) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 34)) | ((value & 0x1UL) << 34);
                }
            }

            
            public ulong SeCreateSymbolicLinkPrivilege
            {
                readonly get
                {
                    return (_bitfield >> 35) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 35)) | ((value & 0x1UL) << 35);
                }
            }

            
            public ulong SeDelegateSessionUserImpersonatePrivilege
            {
                readonly get
                {
                    return (_bitfield >> 36) & 0x1UL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1UL << 36)) | ((value & 0x1UL) << 36);
                }
            }

            public ulong RESERVED2
            {
                readonly get
                {
                    return (_bitfield >> 37) & 0x7FFFFFFUL;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x7FFFFFFUL << 37)) | ((value & 0x7FFFFFFUL) << 37);
                }
            }
        }
    }
}
