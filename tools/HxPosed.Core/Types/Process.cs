using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Types
{
    public partial struct ProcessProtection
    {
        public _Anonymous_e__Union Anonymous;

        public override string ToString() => $"{(ProcessProtectionType)Type} - {(ProcessProtectionSigner)Signer}";

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
            public byte Level;

            [FieldOffset(0)]
            public _Anonymous_1_e__Struct Anonymous;

            public partial struct _Anonymous_1_e__Struct
            {
                public byte _bitfield;

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
    public partial struct ProcessMitigationFlags1
    {
        [FieldOffset(0)]
        public uint MitigationFlags2;

        [FieldOffset(0)]
        public MitigationFlags2Values _MitigationFlags2Values;

        public partial struct MitigationFlags2Values
        {
            public uint _bitfield;

           
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
    public partial struct ProcessMitigationFlags2
    {
        [FieldOffset(0)]
        public uint MitigationFlags;

        [FieldOffset(0)]
        public _MitigationFlagsValues MitigationFlagsValues;

        public partial struct _MitigationFlagsValues
        {
            public uint _bitfield;

           
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

    public partial struct ProcessMitigationFlags
    {
        public ProcessMitigationFlags1 First;
        public ProcessMitigationFlags2 Second;
    }

    public partial struct ProcessSigners
    {
        public byte Level;
        public byte SectionLevel;
    }

    public enum ProcessProtectionType
    {
        None = 0,
        Light = 1,
        Protected = 2,
        Max = 3,
    }

    public enum ProcessProtectionSigner
    {
        None = 0,
        Authenticode = 1,
        CodeGen = 2,
        AntiMalware = 3,
        Lsa = 4,
        Windows = 5,
        WinTcb = 6,
        Max = 7,
    }

    public enum ProcessSignatureLevel
    {
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
        Custom6 = 15,
    }
}
