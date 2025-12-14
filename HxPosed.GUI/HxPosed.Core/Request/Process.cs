using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Request
{
    [StructLayout(LayoutKind.Sequential)]
    public struct OpenProcessRequest
    {
        public uint Id;
        public ObjectOpenType OpenType;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CloseProcessRequest
    {
        public IntPtr Address;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GetProcessFieldRequest
    {
        public IntPtr Address;
        public ProcessField Field;
        public IntPtr Data;
        public ulong DataSize;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct SetProcessFieldRequest
    {
        public IntPtr Address;
        public ProcessField Field;
        public IntPtr Data;
        public ulong DataSize;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GetProcessThreadsRequest
    {
        public IntPtr Address;
        public IntPtr Data;
        public ulong DataSize;
    }

    public enum ProcessField : uint
    {
        Unknown = 0,
        NtPath = 1,
        Protection = 2,
        Signers = 3,
        MitigationFlags = 4,
        Token = 5,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ProcessProtection
    {
        public byte Level;

        public ProcessProtectionType Type
        {
            get
            {
                return (ProcessProtectionType)(Level & 0b_0000_0111);
            }
            set
            {
                Level = (byte)((Level & 0b_1111_1000) | (byte)value);
            }
        }

        public bool Audit
        {
            get => (Level & 0b_0000_1000) == 1;
            set
            {
                Level = (byte)((Level & 0b_1111_0111) | Convert.ToByte(value) << 3);
            }
        }


        public ProcessProtectionSigner Signer
        {
            get
            {
                return (ProcessProtectionSigner)(Level & 0b_1111_0000);
            }
            set
            {
                Level = (byte)((Level & 0b_0000_1111) | (byte)value << 4);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct ProcessSigners
    {
        public ProcessSigningLevel Level;
        public byte SectionLevel;
    }

    [Flags]
    public enum ProcessMitigationFlags : ulong
    {
        ControlFlowGuardEnabled = 1 << 0,
        ControlFlowGuardExportSuppressionEnabled = 1 << 1,
        ControlFlowGuardStrict = 1 << 2,
        DisallowStrippedImages = 1 << 3,
        ForceRelocateImages = 1 << 4,
        HighEntropyASLREnabled = 1 << 5,
        StackRandomizationDisabled = 1 << 6,
        ExtensionPointDisable = 1 << 7,
        DisableDynamicCode = 1 << 8,
        DisableDynamicCodeAllowOptOut = 1 << 9,
        DisableDynamicCodeAllowRemoteDowngrade = 1 << 10,
        AuditDisableDynamicCode = 1 << 11,
        DisallowWin32kSystemCalls = 1 << 12,
        AuditDisallowWin32kSystemCalls = 1 << 13,
        EnableFilteredWin32kAPIs = 1 << 14,
        AuditFilteredWin32kAPIs = 1 << 15,
        DisableNonSystemFonts = 1 << 16,
        AuditNonSystemFontLoading = 1 << 17,
        PreferSystem32Images = 1 << 18,
        ProhibitRemoteImageMap = 1 << 19,
        AuditProhibitRemoteImageMap = 1 << 20,
        ProhibitLowILImageMap = 1 << 21,
        AuditProhibitLowILImageMap = 1 << 22,
        SignatureMitigationOptIn = 1 << 23,
        AuditBlockNonMicrosoftBinaries = 1 << 24,
        AuditBlockNonMicrosoftBinariesAllowStore = 1 << 25,
        LoaderIntegrityContinuityEnabled = 1 << 26,
        AuditLoaderIntegrityContinuity = 1 << 27,
        EnableModuleTamperingProtection = 1 << 28,
        EnableModuleTamperingProtectionNoInherit = 1 << 29,
        RestrictIndirectBranchPrediction = 1 << 30,
        IsolateSecurityDomain = 1U << 31,

        EnableExportAddressFilter = 1L << 32,
        AuditExportAddressFilter = 1L << 33,
        EnableExportAddressFilterPlus = 1L << 34,
        AuditExportAddressFilterPlus = 1L << 35,
        EnableRopStackPivot = 1L << 36,
        AuditRopStackPivot = 1L << 37,
        EnableRopCallerCheck = 1L << 38,
        AuditRopCallerCheck = 1L << 39,
        EnableRopSimExec = 1L << 40,
        AuditRopSimExec = 1L << 41,
        EnableImportAddressFilter = 1L << 42,
        AuditImportAddressFilter = 1L << 43,
        DisablePageCombine = 1L << 44,
        SpeculativeStoreBypassDisable = 1L << 45,
        CetUserShadowStacks = 1L << 46,
        AuditCetUserShadowStacks = 1L << 47,
        AuditCetUserShadowStacksLogged = 1L << 48,
        UserCetSetContextIpValidation = 1L << 49,
        AuditUserCetSetContextIpValidation = 1L << 50,
        AuditUserCetSetContextIpValidationLogged = 1L << 51,
        CetUserShadowStacksStrictMode = 1L << 52,
        BlockNonCetBinaries = 1L << 53,
        BlockNonCetBinariesNonEhcont = 1L << 54,
        AuditBlockNonCetBinaries = 1L << 55,
        AuditBlockNonCetBinariesLogged = 1L << 56,
        XtendedControlFlowGuard_Deprecated = 1L << 57,
        AuditXtendedControlFlowGuard_Deprecated = 1L << 58,
        PointerAuthUserIp = 1L << 59,
        AuditPointerAuthUserIp = 1L << 60,
        AuditPointerAuthUserIpLogged = 1L << 61,
        CetDynamicApisOutOfProcOnly = 1L << 62,
        UserCetSetContextIpValidationRelaxedMode = 1UL << 63
    }


    public enum ProcessSigningLevel : byte
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

    public enum ProcessProtectionType : byte
    {
        None = 0,
        Light = 1,
        Protected = 2,
        Max = 3
    }

    public enum ProcessProtectionSigner : byte
    {
        None = 0,
        Authenticode = 1,
        CodeGen = 2,
        AntiMalware = 3,
        Lsa = 4,
        Windows = 5,
        WinTcb = 7,
        Max = 7
    }
}
