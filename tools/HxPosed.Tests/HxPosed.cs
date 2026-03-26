using System;
using System.Runtime.InteropServices;

namespace HxPosed.PInvoke
{
    public class NativeTypeName(string a) : Attribute
    {

    }

    public enum _HX_OBJECT_TYPES
    {
        HxObHandle = 0,
        HxObProcess = 1,
        HxObThread = 2,
        HxObToken = 3,
        HxObRmd = 4,
        HxObRegKey = 5,
    }

    public enum _HX_OBJECT_STATE
    {
        HxObCreated = 0,
        HxObModified = 1,
        HxObDeleted = 2,
    }

    public partial struct _HX_OBJECT_TYPE
    {
        [NativeTypeName("HX_OBJECT_TYPES")]
        public _HX_OBJECT_TYPES Type;

        [NativeTypeName("HX_OBJECT")]
        public ulong Object;
    }

    public partial struct _HXR_OPEN_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong AddressOrId;
    }

    public partial struct _HXR_CLOSE_OBJECT
    {
        [NativeTypeName("HX_OBJECT")]
        public ulong Address;
    }

    [StructLayout(LayoutKind.Explicit)]
    public partial struct _HX_TOKEN_PRIVILEGES
    {
        [FieldOffset(0)]
        [NativeTypeName("UINT64")]
        public ulong All;

        [FieldOffset(0)]
        [NativeTypeName("__AnonymousRecord_hxposed_L61_C5")]
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

            [NativeTypeName("UINT64 : 2")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 1")]
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

            [NativeTypeName("UINT64 : 27")]
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

    public enum _HX_TOKEN_IMPERSONATION_LEVEL
    {
        Anonymous = 0,
        Identification = 1,
        Impersonation = 2,
        Delegation = 3,
    }

    public enum _HX_TOKEN_TYPE
    {
        HxTokenPrimary = 0,
        HxTokenImpersonation = 1,
    }

    public enum _HX_THREAD_FIELD
    {
        HxThreadFieldActiveImpersonationInfo = 1,
        HxThreadFieldAdjustedClientToken = 2,
    }

    public enum _HX_TOKEN_FIELD
    {
        HxTokenFieldUnknown = 0,
        HxTokenFieldSourceName = 1,
        HxTokenFieldAccountName = 2,
        HxTokenFieldType = 3,
        HxTokenFieldIntegrityLevelIndex = 4,
        HxTokenFieldMandatoryPolicy = 5,
        HxTokenFieldImpersonationLevel = 6,
        HxTokenFieldPresentPrivileges = 7,
        HxTokenFieldEnabledPrivileges = 8,
        HxTokenFieldEnabledByDefaultPrivileges = 9,
    }

    public enum _HX_PROCESS_FIELD
    {
        HxProcessFieldUnknown = 0,
        HxProcessFieldNtPath = 1,
        HxProcessFieldProtection = 2,
        HxProcessFieldSigners = 3,
        HxProcessFieldMitigation = 4,
        HxProcessFieldToken = 5,
        HxProcessFieldThreads = 6,
        HxProcessFieldDirectoryTableBase = 7,
        HxProcessFieldUserDirectoryTableBase = 8,
    }

    public enum _HX_MAP_OPERATION
    {
        HxMemMap = 0,
        HxMemUnMap = 1,
    }

    public enum _HX_MEMORY_POOL
    {
        HxPoolNonPaged = 0,
        HxContiguousPhysical = 1,
    }

    public enum _HX_PAGING_OBJECT
    {
        HxPml5 = 0,
        HxPml4 = 1,
        HxPdp = 2,
        HxPd = 3,
        HxPt = 4,
    }

    public partial struct _HX_VIRTUAL_ADDRESS_FLAGS
    {
        public ulong _bitfield;

        [NativeTypeName("UINT64 : 12")]
        public ulong PhysicalOffset
        {
            readonly get
            {
                return _bitfield & 0xFFFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~0xFFFUL) | (value & 0xFFFUL);
            }
        }

        [NativeTypeName("UINT64 : 9")]
        public ulong PtIndex
        {
            readonly get
            {
                return (_bitfield >> 12) & 0x1FFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x1FFUL << 12)) | ((value & 0x1FFUL) << 12);
            }
        }

        [NativeTypeName("UINT64 : 9")]
        public ulong PdIndex
        {
            readonly get
            {
                return (_bitfield >> 21) & 0x1FFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x1FFUL << 21)) | ((value & 0x1FFUL) << 21);
            }
        }

        [NativeTypeName("UINT64 : 9")]
        public ulong PdpIndex
        {
            readonly get
            {
                return (_bitfield >> 30) & 0x1FFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x1FFUL << 30)) | ((value & 0x1FFUL) << 30);
            }
        }

        [NativeTypeName("UINT64 : 9")]
        public ulong Pml4Index
        {
            readonly get
            {
                return (_bitfield >> 39) & 0x1FFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x1FFUL << 39)) | ((value & 0x1FFUL) << 39);
            }
        }

        [NativeTypeName("UINT64 : 9")]
        public ulong Pml5Index
        {
            readonly get
            {
                return (_bitfield >> 48) & 0x1FFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x1FFUL << 48)) | ((value & 0x1FFUL) << 48);
            }
        }

        [NativeTypeName("UINT64 : 7")]
        public ulong Sign
        {
            readonly get
            {
                return (_bitfield >> 57) & 0x7FUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x7FUL << 57)) | ((value & 0x7FUL) << 57);
            }
        }
    }

    [StructLayout(LayoutKind.Explicit)]
    public unsafe partial struct _HX_VIRTUAL_ADDRESS
    {
        [FieldOffset(0)]
        [NativeTypeName("PVOID")]
        public void* Address;

        [FieldOffset(0)]
        [NativeTypeName("HX_VIRTUAL_ADDRESS_FLAGS")]
        public _HX_VIRTUAL_ADDRESS_FLAGS Indices;
    }

    public partial struct _HX_PAGING_TYPE
    {
        [NativeTypeName("HX_PAGING_OBJECT")]
        public ulong ObjectType;

        [NativeTypeName("HX_VIRTUAL_ADDRESS")]
        public _HX_VIRTUAL_ADDRESS Object;
    }

    public enum _HX_PAGING_OPERATION
    {
        HxPageOperationSet = 0,
        HxPageOperationGet = 1,
    }

    public partial struct _HX_PROCESS_PROTECTION
    {
        [NativeTypeName("__AnonymousRecord_hxposed_L201_C5")]
        public _Anonymous_e__Union Anonymous;

        public override string ToString() => $"{(_HX_PROCESS_PROTECTION_TYPE)Type} - {(_HX_PROCESS_SIGNATURE_LEVEL)Signer}";

        public ref byte Level
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Level, 1));
            }
        }

        public byte Type
        {
            readonly get
            {
                return Anonymous.Anonymous.Type;
            }

            set
            {
                Anonymous.Anonymous.Type = value;
            }
        }

        public byte Audit
        {
            readonly get
            {
                return Anonymous.Anonymous.Audit;
            }

            set
            {
                Anonymous.Anonymous.Audit = value;
            }
        }

        public byte Signer
        {
            readonly get
            {
                return Anonymous.Anonymous.Signer;
            }

            set
            {
                Anonymous.Anonymous.Signer = value;
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("UCHAR")]
            public byte Level;

            [FieldOffset(0)]
            [NativeTypeName("__AnonymousRecord_hxposed_L204_C9")]
            public _Anonymous_1_e__Struct Anonymous;

            public partial struct _Anonymous_1_e__Struct
            {
                public byte _bitfield;

                [NativeTypeName("UCHAR : 3")]
                public byte Type
                {
                    readonly get
                    {
                        return (byte)(_bitfield & 0x7u);
                    }

                    set
                    {
                        _bitfield = (byte)((_bitfield & ~0x7u) | (value & 0x7u));
                    }
                }

                [NativeTypeName("UCHAR : 1")]
                public byte Audit
                {
                    readonly get
                    {
                        return (byte)((_bitfield >> 3) & 0x1u);
                    }

                    set
                    {
                        _bitfield = (byte)((_bitfield & ~(0x1u << 3)) | ((value & 0x1u) << 3));
                    }
                }

                [NativeTypeName("UCHAR : 4")]
                public byte Signer
                {
                    readonly get
                    {
                        return (byte)((_bitfield >> 4) & 0xFu);
                    }

                    set
                    {
                        _bitfield = (byte)((_bitfield & ~(0xFu << 4)) | ((value & 0xFu) << 4));
                    }
                }
            }
        }
    }

    [StructLayout(LayoutKind.Explicit)]
    public partial struct _HX_PROCESS_MITIGATION_FLAGS_2
    {
        [FieldOffset(0)]
        [NativeTypeName("ULONG")]
        public uint MitigationFlags2;

        [FieldOffset(0)]
        [NativeTypeName("struct MitigationFlags2Values")]
        public MitigationFlags2Values _MitigationFlags2Values;

        public partial struct MitigationFlags2Values
        {
            public uint _bitfield;

            [NativeTypeName("ULONG : 1")]
            public uint EnableExportAddressFilter
            {
                readonly get
                {
                    return _bitfield & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~0x1u) | (value & 0x1u);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditExportAddressFilter
            {
                readonly get
                {
                    return (_bitfield >> 1) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 1)) | ((value & 0x1u) << 1);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableExportAddressFilterPlus
            {
                readonly get
                {
                    return (_bitfield >> 2) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 2)) | ((value & 0x1u) << 2);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditExportAddressFilterPlus
            {
                readonly get
                {
                    return (_bitfield >> 3) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 3)) | ((value & 0x1u) << 3);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableRopStackPivot
            {
                readonly get
                {
                    return (_bitfield >> 4) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 4)) | ((value & 0x1u) << 4);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditRopStackPivot
            {
                readonly get
                {
                    return (_bitfield >> 5) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 5)) | ((value & 0x1u) << 5);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableRopCallerCheck
            {
                readonly get
                {
                    return (_bitfield >> 6) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 6)) | ((value & 0x1u) << 6);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditRopCallerCheck
            {
                readonly get
                {
                    return (_bitfield >> 7) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 7)) | ((value & 0x1u) << 7);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableRopSimExec
            {
                readonly get
                {
                    return (_bitfield >> 8) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 8)) | ((value & 0x1u) << 8);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditRopSimExec
            {
                readonly get
                {
                    return (_bitfield >> 9) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 9)) | ((value & 0x1u) << 9);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableImportAddressFilter
            {
                readonly get
                {
                    return (_bitfield >> 10) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 10)) | ((value & 0x1u) << 10);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditImportAddressFilter
            {
                readonly get
                {
                    return (_bitfield >> 11) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 11)) | ((value & 0x1u) << 11);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisablePageCombine
            {
                readonly get
                {
                    return (_bitfield >> 12) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 12)) | ((value & 0x1u) << 12);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint SpeculativeStoreBypassDisable
            {
                readonly get
                {
                    return (_bitfield >> 13) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 13)) | ((value & 0x1u) << 13);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint CetUserShadowStacks
            {
                readonly get
                {
                    return (_bitfield >> 14) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 14)) | ((value & 0x1u) << 14);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditCetUserShadowStacks
            {
                readonly get
                {
                    return (_bitfield >> 15) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 15)) | ((value & 0x1u) << 15);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditCetUserShadowStacksLogged
            {
                readonly get
                {
                    return (_bitfield >> 16) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 16)) | ((value & 0x1u) << 16);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint UserCetSetContextIpValidation
            {
                readonly get
                {
                    return (_bitfield >> 17) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 17)) | ((value & 0x1u) << 17);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditUserCetSetContextIpValidation
            {
                readonly get
                {
                    return (_bitfield >> 18) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 18)) | ((value & 0x1u) << 18);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditUserCetSetContextIpValidationLogged
            {
                readonly get
                {
                    return (_bitfield >> 19) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 19)) | ((value & 0x1u) << 19);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint CetUserShadowStacksStrictMode
            {
                readonly get
                {
                    return (_bitfield >> 20) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 20)) | ((value & 0x1u) << 20);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint BlockNonCetBinaries
            {
                readonly get
                {
                    return (_bitfield >> 21) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 21)) | ((value & 0x1u) << 21);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint BlockNonCetBinariesNonEhcont
            {
                readonly get
                {
                    return (_bitfield >> 22) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 22)) | ((value & 0x1u) << 22);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditBlockNonCetBinaries
            {
                readonly get
                {
                    return (_bitfield >> 23) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 23)) | ((value & 0x1u) << 23);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditBlockNonCetBinariesLogged
            {
                readonly get
                {
                    return (_bitfield >> 24) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 24)) | ((value & 0x1u) << 24);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint XtendedControlFlowGuard_Deprecated
            {
                readonly get
                {
                    return (_bitfield >> 25) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 25)) | ((value & 0x1u) << 25);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditXtendedControlFlowGuard_Deprecated
            {
                readonly get
                {
                    return (_bitfield >> 26) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 26)) | ((value & 0x1u) << 26);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint PointerAuthUserIp
            {
                readonly get
                {
                    return (_bitfield >> 27) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 27)) | ((value & 0x1u) << 27);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditPointerAuthUserIp
            {
                readonly get
                {
                    return (_bitfield >> 28) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 28)) | ((value & 0x1u) << 28);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditPointerAuthUserIpLogged
            {
                readonly get
                {
                    return (_bitfield >> 29) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 29)) | ((value & 0x1u) << 29);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint CetDynamicApisOutOfProcOnly
            {
                readonly get
                {
                    return (_bitfield >> 30) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 30)) | ((value & 0x1u) << 30);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint UserCetSetContextIpValidationRelaxedMode
            {
                readonly get
                {
                    return (_bitfield >> 31) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 31)) | ((value & 0x1u) << 31);
                }
            }
        }
    }

    [StructLayout(LayoutKind.Explicit)]
    public partial struct _HX_PROCESS_MITIGATION_FLAGS_1
    {
        [FieldOffset(0)]
        [NativeTypeName("ULONG")]
        public uint MitigationFlags;

        [FieldOffset(0)]
        [NativeTypeName("struct _MitigationFlagsValues")]
        public _MitigationFlagsValues MitigationFlagsValues;

        public partial struct _MitigationFlagsValues
        {
            public uint _bitfield;

            [NativeTypeName("ULONG : 1")]
            public uint ControlFlowGuardEnabled
            {
                readonly get
                {
                    return _bitfield & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~0x1u) | (value & 0x1u);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ControlFlowGuardExportSuppressionEnabled
            {
                readonly get
                {
                    return (_bitfield >> 1) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 1)) | ((value & 0x1u) << 1);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ControlFlowGuardStrict
            {
                readonly get
                {
                    return (_bitfield >> 2) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 2)) | ((value & 0x1u) << 2);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisallowStrippedImages
            {
                readonly get
                {
                    return (_bitfield >> 3) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 3)) | ((value & 0x1u) << 3);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ForceRelocateImages
            {
                readonly get
                {
                    return (_bitfield >> 4) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 4)) | ((value & 0x1u) << 4);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint HighEntropyASLREnabled
            {
                readonly get
                {
                    return (_bitfield >> 5) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 5)) | ((value & 0x1u) << 5);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint StackRandomizationDisabled
            {
                readonly get
                {
                    return (_bitfield >> 6) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 6)) | ((value & 0x1u) << 6);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ExtensionPointDisable
            {
                readonly get
                {
                    return (_bitfield >> 7) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 7)) | ((value & 0x1u) << 7);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisableDynamicCode
            {
                readonly get
                {
                    return (_bitfield >> 8) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 8)) | ((value & 0x1u) << 8);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisableDynamicCodeAllowOptOut
            {
                readonly get
                {
                    return (_bitfield >> 9) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 9)) | ((value & 0x1u) << 9);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisableDynamicCodeAllowRemoteDowngrade
            {
                readonly get
                {
                    return (_bitfield >> 10) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 10)) | ((value & 0x1u) << 10);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditDisableDynamicCode
            {
                readonly get
                {
                    return (_bitfield >> 11) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 11)) | ((value & 0x1u) << 11);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisallowWin32kSystemCalls
            {
                readonly get
                {
                    return (_bitfield >> 12) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 12)) | ((value & 0x1u) << 12);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditDisallowWin32kSystemCalls
            {
                readonly get
                {
                    return (_bitfield >> 13) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 13)) | ((value & 0x1u) << 13);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableFilteredWin32kAPIs
            {
                readonly get
                {
                    return (_bitfield >> 14) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 14)) | ((value & 0x1u) << 14);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditFilteredWin32kAPIs
            {
                readonly get
                {
                    return (_bitfield >> 15) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 15)) | ((value & 0x1u) << 15);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint DisableNonSystemFonts
            {
                readonly get
                {
                    return (_bitfield >> 16) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 16)) | ((value & 0x1u) << 16);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditNonSystemFontLoading
            {
                readonly get
                {
                    return (_bitfield >> 17) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 17)) | ((value & 0x1u) << 17);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint PreferSystem32Images
            {
                readonly get
                {
                    return (_bitfield >> 18) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 18)) | ((value & 0x1u) << 18);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ProhibitRemoteImageMap
            {
                readonly get
                {
                    return (_bitfield >> 19) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 19)) | ((value & 0x1u) << 19);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditProhibitRemoteImageMap
            {
                readonly get
                {
                    return (_bitfield >> 20) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 20)) | ((value & 0x1u) << 20);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint ProhibitLowILImageMap
            {
                readonly get
                {
                    return (_bitfield >> 21) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 21)) | ((value & 0x1u) << 21);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditProhibitLowILImageMap
            {
                readonly get
                {
                    return (_bitfield >> 22) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 22)) | ((value & 0x1u) << 22);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint SignatureMitigationOptIn
            {
                readonly get
                {
                    return (_bitfield >> 23) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 23)) | ((value & 0x1u) << 23);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditBlockNonMicrosoftBinaries
            {
                readonly get
                {
                    return (_bitfield >> 24) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 24)) | ((value & 0x1u) << 24);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditBlockNonMicrosoftBinariesAllowStore
            {
                readonly get
                {
                    return (_bitfield >> 25) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 25)) | ((value & 0x1u) << 25);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint LoaderIntegrityContinuityEnabled
            {
                readonly get
                {
                    return (_bitfield >> 26) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 26)) | ((value & 0x1u) << 26);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint AuditLoaderIntegrityContinuity
            {
                readonly get
                {
                    return (_bitfield >> 27) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 27)) | ((value & 0x1u) << 27);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableModuleTamperingProtection
            {
                readonly get
                {
                    return (_bitfield >> 28) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 28)) | ((value & 0x1u) << 28);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint EnableModuleTamperingProtectionNoInherit
            {
                readonly get
                {
                    return (_bitfield >> 29) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 29)) | ((value & 0x1u) << 29);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint RestrictIndirectBranchPrediction
            {
                readonly get
                {
                    return (_bitfield >> 30) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 30)) | ((value & 0x1u) << 30);
                }
            }

            [NativeTypeName("ULONG : 1")]
            public uint IsolateSecurityDomain
            {
                readonly get
                {
                    return (_bitfield >> 31) & 0x1u;
                }

                set
                {
                    _bitfield = (_bitfield & ~(0x1u << 31)) | ((value & 0x1u) << 31);
                }
            }
        }
    }

    public partial struct _HX_PROCESS_MITIGATION_FLAGS
    {
        [NativeTypeName("HX_PROCESS_MITIGATION_FLAGS_1")]
        public _HX_PROCESS_MITIGATION_FLAGS_1 First;

        [NativeTypeName("HX_PROCESS_MITIGATION_FLAGS_2")]
        public _HX_PROCESS_MITIGATION_FLAGS_2 Second;
    }

    public partial struct _HX_PROCESS_SIGNERS
    {
        [NativeTypeName("UCHAR")]
        public byte Level;

        [NativeTypeName("UCHAR")]
        public byte SectionLevel;
    }

    public enum _HX_PROCESS_PROTECTION_TYPE
    {
        HxPsProtTypeNone = 0,
        HxPsProtTypeLight = 1,
        HxPsProtTypeProtected = 2,
        HxPsProtTypeMax = 3,
    }

    public enum _HX_PROCESS_PROTECTION_SIGNER
    {
        HxPsProtSigNone = 0,
        HxPsProtSigAuthenticode = 1,
        HxPsProtSigCodeGen = 2,
        HxPsProtSigAntiMalware = 3,
        HxPsProtSigLsa = 4,
        HxPsProtSigWindows = 5,
        HxPsProtSigWinTcb = 6,
        HxPsProtSigMax = 7,
    }

    public enum _HX_PROCESS_SIGNATURE_LEVEL
    {
        HxPsSigUnchecked = 0,
        HxPsSigUnsigned = 1,
        HxPsSigEnterprise = 2,
        HxPsSigCustom = 3,
        HxPsSigAuthenticode = 4,
        HxPsSigCustom2 = 5,
        HxPsSigStore = 6,
        HxPsSigAntiMalware = 7,
        HxPsSigMicrosoft = 8,
        HxPsSigCustom4 = 9,
        HxPsSigCustom5 = 10,
        HxPsSigDynamicCodeGen = 11,
        HxPsSigWindows = 12,
        HxPsSigWindowsPPL = 13,
        HxPsSigWindowsTcb = 14,
        HxPsSigCustom6 = 15,
    }

    public enum _HXS_HYPERVISOR_STATUS
    {
        HxStatusUnknown = 0,
        SystemVirtualized = 1,
        SystemDeVirtualized = 2,
    }

    public partial struct _HXS_OPEN_OBJECT_RESPONSE
    {
        [NativeTypeName("HX_OBJECT_TYPE")]
        public _HX_OBJECT_TYPE Object;
    }

    public partial struct _HXS_STATUS
    {
        [NativeTypeName("HXS_HYPERVISOR_STATUS")]
        public _HXS_HYPERVISOR_STATUS Status;

        [NativeTypeName("UINT32")]
        public uint _PAD;

        [NativeTypeName("UINT32")]
        public uint Version;

        [NativeTypeName("UINT32")]
        public uint _PAD2;
    }

    public partial struct _HXS_GET_SET_PAGE_ATTRIBUTE
    {
        [NativeTypeName("UINT64")]
        public ulong TypeBits;
    }

    public partial struct _HXS_ALLOCATE_MEMORY
    {
        [NativeTypeName("HX_RMD")]
        public ulong RawMemoryDescriptor;
    }

    public partial struct _HXS_DESCRIBE_MEMORY
    {
        [NativeTypeName("HX_RMD")]
        public ulong RawMemoryDescriptor;
    }

    public partial struct _HXS_TRANSLATE_ADDRESS
    {
        [NativeTypeName("UINT64")]
        public ulong PhysicalAddress;
    }

    public partial struct _HXS_GET_PROCESS_FIELD
    {
        [NativeTypeName("HX_PROCESS_FIELD")]
        public ulong Field;

        [NativeTypeName("__AnonymousRecord_hxposed_L384_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref ulong NtPathOffset
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NtPathOffset, 1));
            }
        }

        public ref _HX_PROCESS_PROTECTION Protection
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Protection, 1));
            }
        }

        public ref _HX_PROCESS_SIGNERS Signers
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Signers, 1));
            }
        }

        public ref _HX_PROCESS_MITIGATION_FLAGS MitigationFlags
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.MitigationFlags, 1));
            }
        }

        public ref ulong Token
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Token, 1));
            }
        }

        public ref ulong ThreadsOffset
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.ThreadsOffset, 1));
            }
        }

        public ref ulong DirectoryTableBase
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.DirectoryTableBase, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong NtPathOffset;

            [FieldOffset(0)]
            [NativeTypeName("HX_PROCESS_PROTECTION")]
            public _HX_PROCESS_PROTECTION Protection;

            [FieldOffset(0)]
            [NativeTypeName("HX_PROCESS_SIGNERS")]
            public _HX_PROCESS_SIGNERS Signers;

            [FieldOffset(0)]
            [NativeTypeName("HX_PROCESS_MITIGATION_FLAGS")]
            public _HX_PROCESS_MITIGATION_FLAGS MitigationFlags;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Token;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong ThreadsOffset;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong DirectoryTableBase;
        }
    }

    public unsafe partial struct _HXS_GET_TOKEN_FIELD
    {
        [NativeTypeName("HX_TOKEN_FIELD")]
        public ulong Field;

        [NativeTypeName("__AnonymousRecord_hxposed_L400_C5")]
        public _Anonymous_e__Union Anonymous;

        public Span<sbyte> Name
        {
            get
            {
                return MemoryMarshal.CreateSpan(ref Anonymous.Name[0], 8);
            }
        }

        public ref ulong NameOffset
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NameOffset, 1));
            }
        }

        public ref _HX_TOKEN_TYPE Type
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Type, 1));
            }
        }

        public ref uint IntegrityIndex
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.IntegrityIndex, 1));
            }
        }

        public ref uint Policy
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Policy, 1));
            }
        }

        public ref _HX_TOKEN_IMPERSONATION_LEVEL Impersonationlevel
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Impersonationlevel, 1));
            }
        }

        public ref _HX_TOKEN_PRIVILEGES Privileges
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Privileges, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public unsafe partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("CHAR[8]")]
            public fixed sbyte Name[8];

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong NameOffset;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN_TYPE")]
            public _HX_TOKEN_TYPE Type;

            [FieldOffset(0)]
            [NativeTypeName("UINT32")]
            public uint IntegrityIndex;

            [FieldOffset(0)]
            [NativeTypeName("UINT32")]
            public uint Policy;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN_IMPERSONATION_LEVEL")]
            public _HX_TOKEN_IMPERSONATION_LEVEL Impersonationlevel;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN_PRIVILEGES")]
            public _HX_TOKEN_PRIVILEGES Privileges;
        }
    }

    public partial struct _HXS_GET_THREAD_FIELD
    {
        [NativeTypeName("HX_THREAD_FIELD")]
        public ulong Field;

        [NativeTypeName("__AnonymousRecord_hxposed_L416_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref int ImpersonationStatus
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.ImpersonationStatus, 1));
            }
        }

        public ref ulong Token
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Token, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("BOOL")]
            public int ImpersonationStatus;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN")]
            public ulong Token;
        }
    }

    public partial struct _HXS_CALLBACK_INFORMATION
    {
        [NativeTypeName("HX_OBJECT_TYPE")]
        public _HX_OBJECT_TYPE ObjectType;

        [NativeTypeName("HX_OBJECT_STATE")]
        public _HX_OBJECT_STATE ObjectState;
    }

    public partial struct _HXS_REGISTER_CALLBACK
    {
        [NativeTypeName("HX_CALLBACK")]
        public ulong Object;
    }

    public partial struct _HXS_EXECUTE_PRIVILEGED
    {
        [NativeTypeName("HX_PRIVILEGED_INSTRUCTION")]
        public ulong Instruction;

        [NativeTypeName("__AnonymousRecord_hxposed_L460_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref ulong Cr3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Cr3, 1));
            }
        }

        public ref ulong Cr8
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Cr8, 1));
            }
        }

        public ref ulong Gdt
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Gdt, 1));
            }
        }

        public ref ulong Idt
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Idt, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Cr3;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Cr8;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Gdt;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Idt;
        }
    }

    public partial struct _HXS_MSR_OPERATION
    {
        [NativeTypeName("UINT64")]
        public ulong Msr;
    }

    public partial struct _HXS_GET_HANDLE_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong Object;

        [NativeTypeName("UINT32")]
        public uint GrantedAccess;

        [NativeTypeName("UINT32")]
        public uint _PAD;
    }

    public partial struct _HXR_ALLOCATE_MEMORY
    {
        [NativeTypeName("UINT32")]
        public uint Size;

        [NativeTypeName("UINT32")]
        public uint _PAD;

        [NativeTypeName("HX_MEMORY_POOL")]
        public ulong Pool;
    }

    public partial struct _HXR_FREE_MEMORY
    {
        [NativeTypeName("HX_RMD")]
        public ulong Object;
    }

    public unsafe partial struct _HXR_MAP_RAW_MEMORY_DESCRIPTOR
    {
        [NativeTypeName("HX_RMD")]
        public ulong MemoryDescriptor;

        [NativeTypeName("HX_PROCESS")]
        public ulong AddressSpace;

        [NativeTypeName("PVOID")]
        public void* MapAddress;

        [NativeTypeName("UINT64")]
        public ulong _PAD;

        [NativeTypeName("HX_MAP_OPERATION")]
        public ulong Operation;
    }

    public partial struct _HXR_GET_SET_PAGE_ATTRIBUTE
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong AddressSpace;

        [NativeTypeName("HX_PAGING_OPERATION")]
        public ulong Operation;

        [NativeTypeName("UINT64")]
        public ulong TypeBits;

        [NativeTypeName("UINT64")]
        public ulong _PAD;

        [NativeTypeName("HX_PAGING_TYPE")]
        public _HX_PAGING_TYPE PagingType;
    }

    public partial struct _HXR_TRANSLATE_ADDRESS
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong AddressSpace;

        [NativeTypeName("UINT64")]
        public ulong VirtualAddress;
    }

    public partial struct _HXR_DESCRIBE_MEMORY
    {
        [NativeTypeName("UINT64")]
        public ulong PhysicalAddress;

        [NativeTypeName("UINT32")]
        public uint Size;
    }

    public partial struct _HXR_GET_PROCESS_FIELD
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong Address;

        [NativeTypeName("HXS_GET_PROCESS_FIELD")]
        public _HXS_GET_PROCESS_FIELD Data;
    }

    public partial struct _HXR_SET_PROCESS_FIELD
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong Address;

        [NativeTypeName("HXS_GET_PROCESS_FIELD")]
        public _HXS_GET_PROCESS_FIELD Data;
    }

    public partial struct _HXR_GET_TOKEN_FIELD
    {
        [NativeTypeName("HX_TOKEN")]
        public ulong Address;

        [NativeTypeName("HXS_GET_TOKEN_FIELD")]
        public _HXS_GET_TOKEN_FIELD Data;
    }

    public partial struct _HXR_SET_TOKEN_FIELD
    {
        [NativeTypeName("HX_TOKEN")]
        public ulong Address;

        [NativeTypeName("HXS_GET_TOKEN_FIELD")]
        public _HXS_GET_TOKEN_FIELD Data;
    }

    public partial struct _HXR_GET_THREAD_FIELD
    {
        [NativeTypeName("HX_THREAD")]
        public ulong Address;

        [NativeTypeName("HXS_GET_THREAD_FIELD")]
        public _HXS_GET_THREAD_FIELD Data;
    }

    public partial struct _HXR_SET_THREAD_FIELD
    {
        [NativeTypeName("HX_THREAD")]
        public ulong Address;

        [NativeTypeName("HXS_GET_THREAD_FIELD")]
        public _HXS_GET_THREAD_FIELD Data;
    }

    public unsafe partial struct _HXR_REGISTER_CALLBACK
    {
        [NativeTypeName("HX_OBJECT_TYPE")]
        public _HX_OBJECT_TYPE ObjectType;

        [NativeTypeName("HANDLE")]
        public void* EventHandle;
    }

    public partial struct _HXR_UNREGISTER_CALLBACK
    {
        [NativeTypeName("HX_CALLBACK")]
        public ulong Object;
    }

    public partial struct _HXR_EXECUTE_PRIVILEGED
    {
        [NativeTypeName("HX_PRIVILEGED_INSTRUCTION")]
        public ulong Instruction;

        [NativeTypeName("__AnonymousRecord_hxposed_L580_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref ulong Cr3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Cr3, 1));
            }
        }

        public ref ulong Cr8
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Cr8, 1));
            }
        }

        public ref ulong Gdt
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Gdt, 1));
            }
        }

        public ref ulong Idt
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Idt, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Cr3;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Cr8;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Gdt;

            [FieldOffset(0)]
            [NativeTypeName("UINT64")]
            public ulong Idt;
        }
    }

    public partial struct _HXR_MSR_OPERATION
    {
        [NativeTypeName("UINT64")]
        public ulong Msr;

        [NativeTypeName("UINT64")]
        public ulong Value;

        [NativeTypeName("HX_MSR_OPERATION")]
        public ulong Operation;
    }

    public partial struct _HXR_UPGRADE_HANDLE
    {
        [NativeTypeName("UINT64")]
        public ulong Handle;

        [NativeTypeName("HX_PROCESS")]
        public ulong Process;

        [NativeTypeName("UINT64")]
        public ulong AccessMask;
    }

    public partial struct _HXR_SWAP_HANDLE_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong Handle;

        [NativeTypeName("HX_PROCESS")]
        public ulong Process;

        [NativeTypeName("UINT64")]
        public ulong NewObject;
    }

    public partial struct _HXR_GET_HANDLE_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong Handle;

        [NativeTypeName("HX_PROCESS")]
        public ulong Process;
    }

    public enum _HX_SERVICE_FUNCTION
    {
        HxSvcGetState = 0x0,
        HxSvcOpenProcess = 0x10,
        HxSvcCloseProcess = 0x11,
        HxSvcGetProcessField = 0x12,
        HxSvcSetProcessField = 0x13,
        HxSvcRegisterNotifyEvent = 0x20,
        HxSvcUnregisterNotifyEvent = 0x21,
        HxSvcAllocateMemory = 0x30,
        HxSvcFreeMemory = 0x31,
        HxSvcGetSetPageAttribute = 0x32,
        HxSvcMapRawMemoryDescriptor = 0x33,
        HxSvcTranslateAddress = 0x34,
        HxSvcDescribeMemory = 0x35,
        HxSvcOpenThread = 0x40,
        HxSvcCloseThread = 0x41,
        HxSvcGetThreadField = 0x42,
        HxSvcSetThreadField = 0x43,
        HxSvcOpenToken = 0x50,
        HxSvcCloseToken = 0x51,
        HxSvcGetTokenField = 0x53,
        HxSvcSetTokenField = 0x54,
        HxSvcMsrIo = 0x60,
        HxSvcExecutePrivilegedInstruction = 0x61,
        HxSvcInterProcessorInterrupt = 0x62,
        HxSvcUpgradeHandle = 0x70,
        HxSvcGetHandleObject = 0x71,
        HxSvcSwapHandleObject = 0x72,
    }

    public enum _HX_ERROR_CODE
    {
        HxErrSuccess = 0,
        HxErrNotAllowed = 1,
        HxErrNotFound = 2,
        HxErrInvalidParameters = 3,
        HxErrNtError = 4,
        HxErrTimedOut = 5,
        HxErrHvNotLoaded = 6,
    }

    public enum _HX_NOT_ALLOWED_REASON
    {
        HxErrReasonLockHeld = 2,
        HxErrReasonPageNotPresent = 3,
        HxErrReasonMappingsExist = 4,
        HxErrReasonAccessViolation = 5,
    }

    public enum _HX_NOT_FOUND_REASON
    {
        HxErrReasonProcess = 1,
        HxErrReasonMdl = 3,
        HxErrReasonThread = 4,
        HxErrReasonFunction = 5,
        HxErrReasonToken = 6,
        HxErrReasonCallback = 7,
        HxErrReasonEvent = 9,
        HxErrReasonField = 10,
        HxErrReasonHandle = 11,
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public partial struct _HX_RESULT
    {
        [NativeTypeName("HX_ERROR_CODE")]
        public _HX_ERROR_CODE ErrorCode;

        [NativeTypeName("__AnonymousRecord_hxposed_L685_C5")]
        public _Anonymous_e__Union Anonymous;

        public void ThrowIfError()
        {
            if (ErrorCode != 0)
            {
                var errorText = ErrorCode switch
                {
                   _HX_ERROR_CODE.HxErrSuccess => "Success",
                   _HX_ERROR_CODE.HxErrNotAllowed => "Not allowed",
                   _HX_ERROR_CODE.HxErrNotFound => $"Object {Anonymous.NotFoundReason} not found",
                   _HX_ERROR_CODE.HxErrInvalidParameters => $"Invalid parameter {Anonymous.Parameter} passed",
                   _HX_ERROR_CODE.HxErrNtError => $"NT error {Anonymous.NtStatus}",
                   _HX_ERROR_CODE.HxErrHvNotLoaded => "Hypervisor is not loaded",
                   _HX_ERROR_CODE.HxErrTimedOut => "Timeout"
                };
                throw new Exception(errorText);
            }
        }

        public ref _HX_NOT_ALLOWED_REASON NotAllowedReason
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NotAllowedReason, 1));
            }
        }

        public ref _HX_NOT_FOUND_REASON NotFoundReason
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NotFoundReason, 1));
            }
        }

        public ref uint NtStatus
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NtStatus, 1));
            }
        }

        public ref uint Parameter
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Parameter, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit, Pack = 1)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("enum _HX_NOT_ALLOWED_REASON")]
            public _HX_NOT_ALLOWED_REASON NotAllowedReason;

            [FieldOffset(0)]
            [NativeTypeName("enum _HX_NOT_FOUND_REASON")]
            public _HX_NOT_FOUND_REASON NotFoundReason;

            [FieldOffset(0)]
            [NativeTypeName("UINT32")]
            public uint NtStatus;

            [FieldOffset(0)]
            [NativeTypeName("UINT32")]
            public uint Parameter;
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public partial struct _HX_CALL
    {
        public ulong _bitfield;

        [NativeTypeName("UINT64 : 16")]
        public ulong ServiceFunction
        {
            readonly get
            {
                return _bitfield & 0xFFFFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~0xFFFFUL) | (value & 0xFFFFUL);
            }
        }

        [NativeTypeName("UINT64 : 1")]
        public ulong IgnoreResult
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

        [NativeTypeName("UINT64 : 1")]
        public ulong ExtendedArgsPresent
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

        [NativeTypeName("UINT64 : 46")]
        public ulong Reserved
        {
            readonly get
            {
                return (_bitfield >> 18) & 0x3FFFUL;
            }

            set
            {
                _bitfield = (_bitfield & ~(0x3FFFUL << 18)) | ((value & 0x3FFFUL) << 18);
            }
        }
    }

    public partial struct _HX_REQUEST_RESPONSE
    {
        [NativeTypeName("HX_CALL")]
        public _HX_CALL Call;

        [NativeTypeName("HX_RESULT")]
        public _HX_RESULT Result;

        [NativeTypeName("__AnonymousRecord_hxposed_L704_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref ulong Arg1
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.Arg1, 1));
            }
        }

        public ref ulong Arg2
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.Arg2, 1));
            }
        }

        public ref ulong Arg3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.Arg3, 1));
            }
        }

        public ref ulong Padding
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.Padding, 1));
            }
        }

        public ref UInt128 ExtendedArg1
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.ExtendedArg1, 1));
            }
        }

        public ref UInt128 ExtendedArg2
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.ExtendedArg2, 1));
            }
        }

        public ref UInt128 ExtendedArg3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.ExtendedArg3, 1));
            }
        }

        public ref UInt128 ExtendedArg4
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous.ExtendedArg4, 1));
            }
        }

        public ref _HXS_STATUS StatusResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.StatusResponse, 1));
            }
        }

        public ref _HXS_OPEN_OBJECT_RESPONSE OpenObjectResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.OpenObjectResponse, 1));
            }
        }

        public ref _HXS_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetSetPageAttributeResponse, 1));
            }
        }

        public ref _HXS_ALLOCATE_MEMORY AllocateMemoryResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.AllocateMemoryResponse, 1));
            }
        }

        public ref _HXS_DESCRIBE_MEMORY DescribeMemoryResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.DescribeMemoryResponse, 1));
            }
        }

        public ref _HXS_TRANSLATE_ADDRESS TranslateAddressResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.TranslateAddressResponse, 1));
            }
        }

        public ref _HXS_REGISTER_CALLBACK RegisterCallbackResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.RegisterCallbackResponse, 1));
            }
        }

        public ref _HXS_GET_PROCESS_FIELD GetProcessFieldResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetProcessFieldResponse, 1));
            }
        }

        public ref _HXS_GET_TOKEN_FIELD GetTokenFieldResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetTokenFieldResponse, 1));
            }
        }

        public ref _HXS_GET_THREAD_FIELD GetThreadFieldResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetThreadFieldResponse, 1));
            }
        }

        public ref _HXS_MSR_OPERATION MsrIoResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.MsrIoResponse, 1));
            }
        }

        public ref _HXS_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.ExecutePrivilegedInstructionResponse, 1));
            }
        }

        public ref _HXS_GET_HANDLE_OBJECT GetHandleObjectResponse
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetHandleObjectResponse, 1));
            }
        }

        public ref _HXR_OPEN_OBJECT OpenObjectRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.OpenObjectRequest, 1));
            }
        }

        public ref _HXR_CLOSE_OBJECT CloseObjectRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.CloseObjectRequest, 1));
            }
        }

        public ref _HXR_ALLOCATE_MEMORY AllocateMemoryRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.AllocateMemoryRequest, 1));
            }
        }

        public ref _HXR_FREE_MEMORY FreeMemoryRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.FreeMemoryRequest, 1));
            }
        }

        public ref _HXR_MAP_RAW_MEMORY_DESCRIPTOR MapRawMemoryDescriptorRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.MapRawMemoryDescriptorRequest, 1));
            }
        }

        public ref _HXR_DESCRIBE_MEMORY DescribeMemoryRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.DescribeMemoryRequest, 1));
            }
        }

        public ref _HXR_TRANSLATE_ADDRESS TranslateAddressRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.TranslateAddressRequest, 1));
            }
        }

        public ref _HXR_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetSetPageAttributeRequest, 1));
            }
        }

        public ref _HXR_REGISTER_CALLBACK RegisterCallbackRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.RegisterCallbackRequest, 1));
            }
        }

        public ref _HXR_UNREGISTER_CALLBACK UnregisterCallbackRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.UnregisterCallbackRequest, 1));
            }
        }

        public ref _HXR_GET_PROCESS_FIELD GetProcessFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetProcessFieldRequest, 1));
            }
        }

        public ref _HXR_SET_PROCESS_FIELD SetProcessFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.SetProcessFieldRequest, 1));
            }
        }

        public ref _HXR_GET_TOKEN_FIELD GetTokenFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetTokenFieldRequest, 1));
            }
        }

        public ref _HXR_SET_TOKEN_FIELD SetTokenFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.SetTokenFieldRequest, 1));
            }
        }

        public ref _HXR_GET_THREAD_FIELD GetThreadFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetThreadFieldRequest, 1));
            }
        }

        public ref _HXR_SET_THREAD_FIELD SetThreadFieldRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.SetThreadFieldRequest, 1));
            }
        }

        public ref _HXR_MSR_OPERATION MsrIoRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.MsrIoRequest, 1));
            }
        }

        public ref _HXR_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.ExecutePrivilegedInstructionRequest, 1));
            }
        }

        public ref _HXR_UPGRADE_HANDLE UpgradeHandleRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.UpgradeHandleRequest, 1));
            }
        }

        public ref _HXR_SWAP_HANDLE_OBJECT SwapHandleObjectRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.SwapHandleObjectRequest, 1));
            }
        }

        public ref _HXR_GET_HANDLE_OBJECT GetHandleObjectRequest
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.GetHandleObjectRequest, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit, Pack = 1)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("__AnonymousRecord_hxposed_L705_C9")]
            public _Anonymous_1_e__Struct Anonymous;

            [FieldOffset(0)]
            [NativeTypeName("HXS_STATUS")]
            public _HXS_STATUS StatusResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_OPEN_OBJECT_RESPONSE")]
            public _HXS_OPEN_OBJECT_RESPONSE OpenObjectResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_GET_SET_PAGE_ATTRIBUTE")]
            public _HXS_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_ALLOCATE_MEMORY")]
            public _HXS_ALLOCATE_MEMORY AllocateMemoryResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_DESCRIBE_MEMORY")]
            public _HXS_DESCRIBE_MEMORY DescribeMemoryResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_TRANSLATE_ADDRESS")]
            public _HXS_TRANSLATE_ADDRESS TranslateAddressResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_REGISTER_CALLBACK")]
            public _HXS_REGISTER_CALLBACK RegisterCallbackResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_GET_PROCESS_FIELD")]
            public _HXS_GET_PROCESS_FIELD GetProcessFieldResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_GET_TOKEN_FIELD")]
            public _HXS_GET_TOKEN_FIELD GetTokenFieldResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_GET_THREAD_FIELD")]
            public _HXS_GET_THREAD_FIELD GetThreadFieldResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_MSR_OPERATION")]
            public _HXS_MSR_OPERATION MsrIoResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_EXECUTE_PRIVILEGED")]
            public _HXS_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXS_GET_HANDLE_OBJECT")]
            public _HXS_GET_HANDLE_OBJECT GetHandleObjectResponse;

            [FieldOffset(0)]
            [NativeTypeName("HXR_OPEN_OBJECT")]
            public _HXR_OPEN_OBJECT OpenObjectRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_CLOSE_OBJECT")]
            public _HXR_CLOSE_OBJECT CloseObjectRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_ALLOCATE_MEMORY")]
            public _HXR_ALLOCATE_MEMORY AllocateMemoryRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_FREE_MEMORY")]
            public _HXR_FREE_MEMORY FreeMemoryRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_MAP_RAW_MEMORY_DESCRIPTOR")]
            public _HXR_MAP_RAW_MEMORY_DESCRIPTOR MapRawMemoryDescriptorRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_DESCRIBE_MEMORY")]
            public _HXR_DESCRIBE_MEMORY DescribeMemoryRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_TRANSLATE_ADDRESS")]
            public _HXR_TRANSLATE_ADDRESS TranslateAddressRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_GET_SET_PAGE_ATTRIBUTE")]
            public _HXR_GET_SET_PAGE_ATTRIBUTE GetSetPageAttributeRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_REGISTER_CALLBACK")]
            public _HXR_REGISTER_CALLBACK RegisterCallbackRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_UNREGISTER_CALLBACK")]
            public _HXR_UNREGISTER_CALLBACK UnregisterCallbackRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_GET_PROCESS_FIELD")]
            public _HXR_GET_PROCESS_FIELD GetProcessFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_SET_PROCESS_FIELD")]
            public _HXR_SET_PROCESS_FIELD SetProcessFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_GET_TOKEN_FIELD")]
            public _HXR_GET_TOKEN_FIELD GetTokenFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_SET_TOKEN_FIELD")]
            public _HXR_SET_TOKEN_FIELD SetTokenFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_GET_THREAD_FIELD")]
            public _HXR_GET_THREAD_FIELD GetThreadFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_SET_THREAD_FIELD")]
            public _HXR_SET_THREAD_FIELD SetThreadFieldRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_MSR_OPERATION")]
            public _HXR_MSR_OPERATION MsrIoRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_EXECUTE_PRIVILEGED")]
            public _HXR_EXECUTE_PRIVILEGED ExecutePrivilegedInstructionRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_UPGRADE_HANDLE")]
            public _HXR_UPGRADE_HANDLE UpgradeHandleRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_SWAP_HANDLE_OBJECT")]
            public _HXR_SWAP_HANDLE_OBJECT SwapHandleObjectRequest;

            [FieldOffset(0)]
            [NativeTypeName("HXR_GET_HANDLE_OBJECT")]
            public _HXR_GET_HANDLE_OBJECT GetHandleObjectRequest;

            [StructLayout(LayoutKind.Sequential, Pack = 1)]
            public partial struct _Anonymous_1_e__Struct
            {
                [NativeTypeName("UINT64")]
                public ulong Arg1;

                [NativeTypeName("UINT64")]
                public ulong Arg2;

                [NativeTypeName("UINT64")]
                public ulong Arg3;

                [NativeTypeName("UINT64")]
                public ulong Padding;

                [NativeTypeName("__uint128_t")]
                public UInt128 ExtendedArg1;

                [NativeTypeName("__uint128_t")]
                public UInt128 ExtendedArg2;

                [NativeTypeName("__uint128_t")]
                public UInt128 ExtendedArg3;

                [NativeTypeName("__uint128_t")]
                public UInt128 ExtendedArg4;
            }
        }
    }

    public static unsafe partial class Methods
    {
        public const int HxMsrRead = 0;
        public const int HxMsrWrite = 1;

        public const int HxPiHlt = 0;
        public const int HxPiMovToCr8 = 1;
        public const int HxPiMovToCr3 = 2;
        public const int HxPiMovFromCr8 = 3;
        public const int HxPiMovFromCr3 = 4;
        public const int HxPiLgdt = 5;
        public const int HxPiLidt = 6;
        public const int HxPiSgdt = 7;
        public const int HxPiSidt = 8;
        public const int HxPiCli = 9;
        public const int HxPiSti = 10;

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxpTrap([NativeTypeName("PHX_REQUEST_RESPONSE")] _HX_REQUEST_RESPONSE* RequestResponse);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("UINT32")]
        public static extern uint HxReadAsyncResponseLength([NativeTypeName("UINT64")] ulong Offset);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("PVOID")]
        public static extern void* HxReadAsyncResponseSlice([NativeTypeName("UINT64")] ulong Offset, [NativeTypeName("PUINT32")] uint* Length);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("PVOID")]
        public static extern void* HxReadAsyncResponseType([NativeTypeName("UINT64")] ulong Offset);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("BOOL")]
        public static extern int HxGetStatus([NativeTypeName("PHXS_STATUS")] _HXS_STATUS* Response);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxCloseObject([NativeTypeName("HX_SERVICE_FUNCTION")] _HX_SERVICE_FUNCTION Function, [NativeTypeName("HX_OBJECT")] ulong Object);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxOpenObject([NativeTypeName("HX_SERVICE_FUNCTION")] _HX_SERVICE_FUNCTION Function, [NativeTypeName("PVOID")] void* AddrOrId, [NativeTypeName("PHX_OBJECT")] ulong* Object);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessProtection([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_PROTECTION")] _HX_PROCESS_PROTECTION* Protection);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetProcessProtection([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_PROTECTION")] _HX_PROCESS_PROTECTION* Protection);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessMitigation([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_MITIGATION_FLAGS")] _HX_PROCESS_MITIGATION_FLAGS* Mitigation);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetProcessMitigation([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_MITIGATION_FLAGS")] _HX_PROCESS_MITIGATION_FLAGS* Mitigation);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessSigners([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_SIGNERS")] _HX_PROCESS_SIGNERS* Signers);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetProcessSigners([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PHX_PROCESS_SIGNERS")] _HX_PROCESS_SIGNERS* Signers);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessToken([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PUINT64")] ulong* Token);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetProcessToken([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PUINT64")] ulong* Token);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessDirectoryTableBase([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PUINT64")] ulong* DirectoryTableBase);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetProcessDirectoryTableBase([NativeTypeName("HX_OBJECT")] ulong Process, [NativeTypeName("PUINT64")] ulong* DirectoryTableBase);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessNtPath([NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("PWCHAR *")] ushort** Name);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetProcessThreads([NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("PUINT32 *")] uint** Threads, [NativeTypeName("PUINT32")] uint* Count);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenSourceName([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PCHAR")] sbyte* SourceName);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenSourceName([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PCHAR")] sbyte* SourceName);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenType([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_TYPE")] _HX_TOKEN_TYPE* Type);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenType([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_TYPE")] _HX_TOKEN_TYPE* Type);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenIntegrityLevelIndex([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PUINT32")] uint* IntegrityLevelIndex);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenIntegrityLevelIndex([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PUINT32")] uint* IntegrityLevelIndex);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenMandatoryPolicy([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PUINT32")] uint* MandatoryPolicy);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenMandatoryPolicy([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PUINT32")] uint* MandatoryPolicy);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenImpersonationLevel([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_IMPERSONATION_LEVEL")] _HX_TOKEN_IMPERSONATION_LEVEL* ImpersonationLevel);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenImpersonationLevel([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_IMPERSONATION_LEVEL")] _HX_TOKEN_IMPERSONATION_LEVEL* ImpersonationLevel);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenPresentPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* PresentPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenPresentPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* PresentPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenEnabledPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* EnabledPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenEnabledPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* EnabledPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenEnabledByDefaultPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* EnabledByDefaultPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetTokenEnabledByDefaultPrivileges([NativeTypeName("HX_OBJECT")] ulong Token, [NativeTypeName("PHX_TOKEN_PRIVILEGES")] _HX_TOKEN_PRIVILEGES* EnabledByDefaultPrivileges);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetTokenAccountName([NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("PWCHAR *")] ushort** Name);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetThreadActiveImpersonationInfo([NativeTypeName("HX_OBJECT")] ulong Thread, [NativeTypeName("PBOOL")] int* ActiveImpersonationInfo);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetThreadActiveImpersonationInfo([NativeTypeName("HX_OBJECT")] ulong Thread, [NativeTypeName("PBOOL")] int* ActiveImpersonationInfo);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetThreadAdjustedClientToken([NativeTypeName("HX_OBJECT")] ulong Thread, [NativeTypeName("PHX_TOKEN")] ulong* AdjustedClientToken);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSetThreadAdjustedClientToken([NativeTypeName("HX_OBJECT")] ulong Thread, [NativeTypeName("PHX_TOKEN")] ulong* AdjustedClientToken);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxReadMsr([NativeTypeName("UINT64")] ulong Msr, [NativeTypeName("PUINT64")] ulong* Value);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxWriteMsr([NativeTypeName("UINT64")] ulong Msr, [NativeTypeName("UINT64")] ulong Value);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxExecPrivileged([NativeTypeName("HX_PRIVILEGED_INSTRUCTION")] ulong Instruction, [NativeTypeName("PUINT64")] ulong* Result);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxUpgradeHandle([NativeTypeName("UINT64")] ulong Handle, [NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("UINT32")] uint AccessMask);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxSwapHandleObject([NativeTypeName("UINT64")] ulong Handle, [NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("HX_OBJECT")] ulong NewObject);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxGetHandleObject([NativeTypeName("UINT64")] ulong Handle, [NativeTypeName("HX_PROCESS")] ulong Process, [NativeTypeName("PHX_OBJECT")] ulong* Object, [NativeTypeName("PUINT32")] uint* GrantedAccess);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxAllocateMemory([NativeTypeName("HX_MEMORY_POOL")] ulong Pool, [NativeTypeName("UINT32")] uint Size, [NativeTypeName("PHX_RMD")] ulong* Descriptor);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxFreeMemory([NativeTypeName("HX_RMD")] ulong Descriptor);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxMapDescriptor([NativeTypeName("HX_RMD")] ulong Descriptor, [NativeTypeName("HX_PROCESS")] ulong AddressSpace, [NativeTypeName("PVOID")] void* MapAddress, [NativeTypeName("HX_MAP_OPERATION")] ulong Operation);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxDescribeMemory([NativeTypeName("UINT64")] ulong PhysicalAddress, [NativeTypeName("UINT32")] uint Size, [NativeTypeName("PHX_RMD")] ulong* Descriptor);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: NativeTypeName("HX_RESULT")]
        public static extern _HX_RESULT HxTranslateAddress([NativeTypeName("PVOID")] void* VirtualAddress, [NativeTypeName("HX_PROCESS")] ulong AddressSpace, [NativeTypeName("PUINT64")] ulong* PhysicalAddress);
    }
}
