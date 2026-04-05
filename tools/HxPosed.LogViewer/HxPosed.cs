using Microsoft.Win32.SafeHandles;
using System;
using System.Runtime.InteropServices;
using System.Security.Policy;
using static HxPosed.PInvoke._HX_REQUEST_RESPONSE;

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

    public unsafe partial struct _HX_OBJECT_TYPE
    {
        [NativeTypeName("HX_OBJECT_TYPES")]
        public _HX_OBJECT_TYPES Type;

        [NativeTypeName("PVOID")]
        public ulong Object;
    }

    public partial struct _HXR_OPEN_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong AddressOrId;

        [NativeTypeName("HX_OPEN_TYPE")]
        public ulong OpenType;
    }

    public partial struct _HXR_CLOSE_OBJECT
    {
        [NativeTypeName("UINT64")]
        public ulong Address;
    }

    [StructLayout(LayoutKind.Explicit)]
    public partial struct _HX_TOKEN_PRIVILEGES
    {
        [FieldOffset(0)]
        [NativeTypeName("UINT64")]
        public ulong All;

        [FieldOffset(0)]
        [NativeTypeName("__AnonymousRecord_hxposed_L55_C5")]
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
        HxTokenPrimary,
        HxTokenImpersonation,
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
        public ulong Address;

        [FieldOffset(0)]
        [NativeTypeName("HX_VIRTUAL_ADDRESS_FLAGS")]
        public _HX_VIRTUAL_ADDRESS_FLAGS Indices;
    }

    public partial struct _HX_PAGING_TYPE
    {
        [NativeTypeName("HX_PAGING_OBJECT")]
        public ulong ObjectType;

        [NativeTypeName("UINT64")]
        public ulong PAD;

        [NativeTypeName("HX_VIRTUAL_ADDRESS")]
        public _HX_VIRTUAL_ADDRESS Object;
    }

    public partial struct _HX_PROCESS_PROTECTION
    {
        public override string ToString() => $"{(_HX_PROCESS_PROTECTION_TYPE)Type} - {(_HX_PROCESS_PROTECTION_SIGNER)Signer}";

        [NativeTypeName("__AnonymousRecord_hxposed_L198_C5")]
        public _Anonymous_e__Union Anonymous;

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
                return Anonymous.Anonymous_1.Type;
            }

            set
            {
                Anonymous.Anonymous_1.Type = value;
            }
        }

        public byte Audit
        {
            readonly get
            {
                return Anonymous.Anonymous_1.Audit;
            }

            set
            {
                Anonymous.Anonymous_1.Audit = value;
            }
        }

        public byte Signer
        {
            readonly get
            {
                return Anonymous.Anonymous_1.Signer;
            }

            set
            {
                Anonymous.Anonymous_1.Signer = value;
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("UCHAR")]
            public byte Level;

            [FieldOffset(0)]
            [NativeTypeName("__AnonymousRecord_hxposed_L201_C9")]
            public _Anonymous_1_e__Struct Anonymous_1;

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

    public unsafe partial struct _HXS_OPEN_OBJECT_RESPONSE
    {
        [NativeTypeName("_HX_OBJECT_TYPE")]
        public _HX_OBJECT_TYPE Object;
    }

    public partial struct _HXS_STATUS
    {
        [NativeTypeName("HXS_HYPERVISOR_STATUS")]
        public _HXS_HYPERVISOR_STATUS Status;

        [NativeTypeName("UINT64")]
        public ulong Version;
    }

    public partial struct _HXS_GET_SET_PAGE_ATTRIBUTE
    {
        [NativeTypeName("UINT64")]
        public ulong TypeBits;
    }

    public unsafe partial struct _HXS_ALLOCATE_MEMORY
    {
        [NativeTypeName("HX_RMD")]
        public ulong RawMemoryDescriptor;
    }

    public unsafe partial struct _HXS_DESCRIBE_MEMORY
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
        public HxProcessField Field;

        [NativeTypeName("__AnonymousRecord_hxposed_L380_C5")]
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

        [NativeTypeName("__AnonymousRecord_hxposed_L396_C5")]
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

        public ref uint Index
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Index, 1));
            }
        }

        public ref uint Policy
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Policy, 1));
            }
        }

        public ref _HX_TOKEN_IMPERSONATION_LEVEL Level
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Level, 1));
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
            public uint Index;

            [FieldOffset(0)]
            [NativeTypeName("UINT32")]
            public uint Policy;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN_IMPERSONATION_LEVEL")]
            public _HX_TOKEN_IMPERSONATION_LEVEL Level;

            [FieldOffset(0)]
            [NativeTypeName("HX_TOKEN_PRIVILEGES")]
            public _HX_TOKEN_PRIVILEGES Privileges;
        }
    }

    public unsafe partial struct _HXS_GET_THREAD_FIELD
    {
        [NativeTypeName("HX_THREAD_FIELD")]
        public ulong Field;

        [NativeTypeName("__AnonymousRecord_hxposed_L412_C5")]
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
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref this, 1)).Anonymous.Token;
            }
        }

        [StructLayout(LayoutKind.Explicit)]
        public unsafe partial struct _Anonymous_e__Union
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

    public unsafe partial struct _HXS_REGISTER_CALLBACK
    {
        [NativeTypeName("HX_CALLBACK")]
        public ulong Object;
    }

    public partial struct _HXS_EXECUTE_PRIVILEGED
    {
        [NativeTypeName("HX_PRIVILEGED_INSTRUCTION")]
        public ulong Instruction;

        [NativeTypeName("__AnonymousRecord_hxposed_L456_C5")]
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

    public partial struct _HXR_ALLOCATE_MEMORY
    {
        [NativeTypeName("UINT32")]
        public uint Size;

        [NativeTypeName("UINT32")]
        public uint PAD;

        [NativeTypeName("HX_MEMORY_POOL")]
        public ulong Pool;
    }

    public unsafe partial struct _HXR_FREE_MEMORY
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
        public ulong MapAddress;

        [NativeTypeName("UINT64")]
        public ulong _PAD;

        [NativeTypeName("HX_MAP_OPERATION")]
        public ulong Operation;
    }

    public unsafe partial struct _HXR_GET_SET_PAGE_ATTRIBUTE
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

    public unsafe partial struct _HXR_TRANSLATE_ADDRESS
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

    public unsafe partial struct _HXR_KILL_PROCESS
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong Address;

        [NativeTypeName("UINT32")]
        public uint ExitCode;

        [NativeTypeName("UINT32")]
        public uint PAD;
    }

    public unsafe partial struct _HXR_GET_PROCESS_FIELD
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong Address;

        [NativeTypeName("HXS_GET_PROCESS_FIELD")]
        public _HXS_GET_PROCESS_FIELD Data;
    }

    public unsafe partial struct _HXR_SET_PROCESS_FIELD
    {
        [NativeTypeName("HX_PROCESS")]
        public ulong Address;

        [NativeTypeName("HXS_GET_PROCESS_FIELD")]
        public _HXS_GET_PROCESS_FIELD Data;
    }

    public unsafe partial struct _HXR_GET_TOKEN_FIELD
    {
        [NativeTypeName("HX_TOKEN")]
        public ulong Address;

        [NativeTypeName("HXS_GET_TOKEN_FIELD")]
        public _HXS_GET_TOKEN_FIELD Data;
    }

    public unsafe partial struct _HXR_SET_TOKEN_FIELD
    {
        [NativeTypeName("HX_TOKEN")]
        public ulong Address;

        [NativeTypeName("HXS_GET_TOKEN_FIELD")]
        public _HXS_GET_TOKEN_FIELD Data;
    }

    public unsafe partial struct _HXR_GET_THREAD_FIELD
    {
        [NativeTypeName("HX_THREAD")]
        public ulong Address;

        [NativeTypeName("HXS_GET_THREAD_FIELD")]
        public _HXS_GET_THREAD_FIELD Data;
    }

    public unsafe partial struct _HXR_SET_THREAD_FIELD
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
        public ulong EventHandle;
    }

    public unsafe partial struct _HXR_UNREGISTER_CALLBACK
    {
        [NativeTypeName("HX_CALLBACK")]
        public ulong Object;
    }

    public partial struct _HXR_EXECUTE_PRIVILEGED
    {
        [NativeTypeName("HX_PRIVILEGED_INSTRUCTION")]
        public ulong Instruction;

        [NativeTypeName("__AnonymousRecord_hxposed_L574_C5")]
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

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public partial struct _HX_RESULT
    {
        [NativeTypeName("UINT32")]
        public uint ErrorCode;

        [NativeTypeName("UINT32")]
        public uint ErrorReason;
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

        [NativeTypeName("__AnonymousRecord_hxposed_L640_C5")]
        public _Anonymous_e__Union Anonymous;

        public ref ulong Arg1
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.Arg1, 1));
            }
        }

        public ref ulong Arg2
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.Arg2, 1));
            }
        }

        public ref ulong Arg3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.Arg3, 1));
            }
        }

        public ref ulong Padding
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.Padding, 1));
            }
        }

        public ref UInt128 ExtendedArg1
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.ExtendedArg1, 1));
            }
        }

        public ref UInt128 ExtendedArg2
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.ExtendedArg2, 1));
            }
        }

        public ref UInt128 ExtendedArg3
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.ExtendedArg3, 1));
            }
        }

        public ref UInt128 ExtendedArg4
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Anonymous_1.ExtendedArg4, 1));
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

        [StructLayout(LayoutKind.Explicit, Pack = 1)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            [NativeTypeName("__AnonymousRecord_hxposed_L641_C9")]
            public _Anonymous_1_e__Struct Anonymous_1;

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

        public enum HxThreadField : ulong
        {
            ActiveImpersonationInfo = 1,
            AdjustedClientToken = 2
        }

        public enum HxTokenField : ulong
        {
            Unknown = 0,
            SourceName = 1,
            AccountName = 2,
            Type = 3,
            IntegrityLevelIndex = 4,
            MandatoryPolicy = 5,
            ImpersonationLevel = 6,
            PresentPrivileges = 7,
            EnabledPrivileges = 8,
            EnabledByDefaultPrivileges = 9
        }

        public enum HxProcessField : ulong
        {
            Unknown = 0,
            NtPath = 1,
            Protection = 2,
            Signers = 3,
            MitigationFlags = 4,
            Token = 5,
            Threads = 6,
            DirectoryTableBase = 7,
            UserDirectoryTableBase = 8
        }
    }

    public static unsafe partial class Methods
    {
        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        public static extern nint CreateEvent(
    IntPtr lpEventAttributes,
    bool bManualReset,
    bool bInitialState,
    string lpName);

        public const int HxOpenHandle = 0;
        public const int HxOpenHypervisor = 1;

        public const int HxMemMap = 0;
        public const int HxMemUnMap = 1;

        public const int HxPoolNonPaged = 0;
        public const int HxContiguousPhysical = 1;

        public const int HxPml5 = 0;
        public const int HxPml4 = 1;
        public const int HxPdp = 2;
        public const int HxPd = 3;
        public const int HxPt = 4;

        public const int HxPageOperationSet = 0;
        public const int HxPageOperationGet = 1;

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

        [DllImport("libhxposed.dll")]
        [return: NativeTypeName("BOOL")]
        public static extern int HxGetStatus([NativeTypeName("PHXS_STATUS")] _HXS_STATUS* Response);

        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl)]
        public static extern int HxpTrap([NativeTypeName("PHX_REQUEST_RESPONSE")] _HX_REQUEST_RESPONSE* RequestResponse);

        [DllImport("libhxposed.dll")]
        [return: NativeTypeName("UINT32")]
        public static extern uint HxReadAsyncResponseLength([NativeTypeName("UINT64")] ulong Offset);

        [DllImport("libhxposed.dll")]
        [return: NativeTypeName("PVOID")]
        public static extern ulong HxReadAsyncResponseSlice([NativeTypeName("UINT64")] ulong Offset, [NativeTypeName("PUINT32")] uint* Count);

        [DllImport("libhxposed.dll")]
        [return: NativeTypeName("PVOID")]
        public static extern ulong HxReadAsyncResponseType([NativeTypeName("UINT64")] ulong Offset);

        [DllImport("ntdll.dll")]
        public static extern uint NtQuerySystemInformation(
    int SystemInformationClass,
    IntPtr SystemInformation,
    int SystemInformationLength,
    out int ReturnLength);

        [StructLayout(LayoutKind.Sequential)]
        public struct UNICODE_STRING
        {
            public ushort Length;
            public ushort MaximumLength;
            public IntPtr Buffer;
        }


        [StructLayout(LayoutKind.Sequential)]
        public struct SYSTEM_PROCESS_INFORMATION
        {
            public uint NextEntryOffset;
            public uint NumberOfThreads;
            public long WorkingSetPrivateSize;
            public uint HardFaultCount;
            public uint NumberOfThreadsHighWatermark;
            public ulong CycleTime;
            public long CreateTime;
            public long UserTime;
            public long KernelTime;
            public UNICODE_STRING ImageName;
            public int BasePriority;
            public IntPtr UniqueProcessId;
            public IntPtr InheritedFromUniqueProcessId;
            public uint HandleCount;
            public uint SessionId;
            public UIntPtr UniqueProcessKey;
            public UIntPtr PeakVirtualSize;
            public UIntPtr VirtualSize;
            public uint PageFaultCount;
            public UIntPtr PeakWorkingSetSize;
            public UIntPtr WorkingSetSize;
            public UIntPtr QuotaPeakPagedPoolUsage;
            public UIntPtr QuotaPagedPoolUsage;
            public UIntPtr QuotaPeakNonPagedPoolUsage;
            public UIntPtr QuotaNonPagedPoolUsage;
            public UIntPtr PagefileUsage;
            public UIntPtr PeakPagefileUsage;
            public UIntPtr PrivatePageCount;
            public long ReadOperationCount;
            public long WriteOperationCount;
            public long OtherOperationCount;
            public long ReadTransferCount;
            public long WriteTransferCount;
            public long OtherTransferCount;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct SYSTEM_THREAD_INFORMATION
        {
            public long KernelTime;
            public long UserTime;
            public long CreateTime;
            public uint WaitTime;
            public IntPtr StartAddress;
            public CLIENT_ID ClientId;
            public int Priority;
            public int BasePriority;
            public uint ContextSwitches;
            public uint ThreadState;
            public uint WaitReason;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct CLIENT_ID
        {
            public IntPtr UniqueProcess;
            public IntPtr UniqueThread;
        }

        public const int SystemProcessInformation = 5;
    }
}
