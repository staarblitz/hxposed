global using HxCallback = nint;
global using HxObject = nint;
global using HxProcess = nint;
global using HxRmd = nint;
global using HxThread = nint;
global using HxToken = nint;
using HxPosed.Core.Exceptions;
using HxPosed.Core.Objects;
using HxPosed.Core.Types;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core
{
    public partial class HxPosed
    {
        private const string LIB_NAME = "libhxposed.dll";

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern bool HxGetStatus(ref HxStatus status);

        public static void CloseObject(ServiceFunction svc, HxObject obj)
        {
            HxCloseObject(svc, obj).ThrowIfError();
        }

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern HxResult HxCloseObject(ServiceFunction svc, HxObject obj);

        public static HxObject OpenObject(ServiceFunction svc, nint addr)
        {
            var obj = new HxObject();
            HxOpenObject(svc, addr, ref obj).ThrowIfError();
            return obj;
        }

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern HxResult HxOpenObject(ServiceFunction svc, nint addrOrId, ref HxObject obj);

        public static string GetProcessNtPath(HxProcess process)
        {
            var buf = Marshal.AllocHGlobal(512);
            var size = 0L;
            HxGetProcessNtPath(process, buf, ref size).ThrowIfError();
            var str = Marshal.PtrToStringUni(buf);
            Marshal.FreeHGlobal(buf);
            return str!;
        }

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern HxResult HxGetProcessNtPath(HxProcess process, nint name, ref long size);

        public static Span<int> GetProcessThreaads(HxProcess process)
        {
            Span<int> span = new int[256];
            var size = 0L;
            HxGetProcessThreads(process, ref MemoryMarshal.GetReference(span), ref size).ThrowIfError();
            return span[..(int)size];
        }

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern HxResult HxGetProcessThreads(HxProcess process, ref int threads, ref long size);

        public static string GetTokenAccountName(HxToken token)
        {
            var buf = Marshal.AllocHGlobal(512);
            var size = 0L;
            HxGetTokenAccountName(token, buf, ref size).ThrowIfError();
            var str = Marshal.PtrToStringUni(buf);
            Marshal.FreeHGlobal(buf);
            return str!;
        }

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        private static extern HxResult HxGetTokenAccountName(HxToken token, nint name, ref long size);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetProcessProtection(HxProcess process, ref ProcessProtection Protection);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetProcessProtection(HxProcess process, ref ProcessProtection Protection);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetProcessMitigation(HxProcess process, ref ProcessMitigationFlags Mitigation);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetProcessMitigation(HxProcess process, ref ProcessMitigationFlags Mitigation);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetProcessSigners(HxProcess process, ref ProcessSigners Signers);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetProcessSigners(HxProcess process, ref ProcessSigners Signers);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetProcessToken(HxProcess process, ref HxToken token);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetProcessToken(HxProcess process, ref HxToken token);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetProcessDirectoryTableBase(HxProcess process, ref ulong DirectoryTableBase);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetProcessDirectoryTableBase(HxProcess process, ref ulong DirectoryTableBase);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenSourceName(HxToken token, ref ulong SourceName);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenType(HxToken token, ref TokenType Type);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenType(HxToken token, ref TokenType Type);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenIntegrityLevelIndex(HxToken token, ref uint IntegrityLevelIndex);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenIntegrityLevelIndex(HxToken token, ref uint IntegrityLevelIndex);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenMandatoryPolicy(HxToken token, ref uint MandatoryPolicy);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenMandatoryPolicy(HxToken token, ref uint MandatoryPolicy);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenImpersonationLevel(HxToken token, ref TokenImpersonationLevel ImpersonationLevel);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenImpersonationLevel(HxToken token, ref TokenImpersonationLevel ImpersonationLevel);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenPresentPrivileges(HxToken token, ref TokenPrivileges PresentPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenPresentPrivileges(HxToken token, ref TokenPrivileges PresentPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenEnabledPrivileges(HxToken token, ref TokenPrivileges EnabledPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenEnabledPrivileges(HxToken token, ref TokenPrivileges EnabledPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetTokenEnabledByDefaultPrivileges(HxToken token, ref TokenPrivileges EnabledByDefaultPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetTokenEnabledByDefaultPrivileges(HxToken token, ref TokenPrivileges EnabledByDefaultPrivileges);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetThreadActiveImpersonationInfo(HxThread Thread, ref bool ActiveImpersonationInfo);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetThreadActiveImpersonationInfo(HxThread Thread, ref bool ActiveImpersonationInfo);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetThreadAdjustedClientToken(HxThread Thread, ref ulong AdjustedClientToken);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSetThreadAdjustedClientToken(HxThread Thread, ref ulong AdjustedClientToken);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxReadMsr(ulong Msr, ref ulong Value);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxWriteMsr(ulong Msr, ulong Value);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxExecPrivileged(ulong Instruction, ref ulong Result);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxUpgradeHandle(ulong Handle, HxProcess process, uint AccessMask);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxSwapHandleObject(ulong Handle, HxProcess process, nint NewObject);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxGetHandleObject(ulong Handle, HxProcess process, ref ulong nint, ref uint GrantedAccess);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxAllocateMemory(ulong Pool, uint Size, ref ulong Descriptor);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxFreeMemory(ulong Descriptor);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxMapDescriptor(ulong Descriptor, ulong AddressSpace, nint MapAddress, ulong Operation);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxDescribeMemory(ulong PhysicalAddress, uint Size, ref ulong Descriptor);

        [DllImport(LIB_NAME, CallingConvention = CallingConvention.Cdecl)]
        public static extern HxResult HxTranslateAddress(nint VirtualAddress, ulong AddressSpace, ref ulong PhysicalAddress);
    }
}
