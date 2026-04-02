using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core
{
    public static class Win32
    {
        public const int HANDLE_ALL_ACCESS = 0x1FFFFFF;
        public const int PROCESS_QUERY_INFORMATION = 0x400;
        public const int THREAD_QUERY_INFORMATION = 0x0040;

        [DllImport("kernel32.dll")]
        public static extern int GetCurrentProcessId();
        [DllImport("kernel32.dll")]
        public static extern int GetCurrentThreadId();

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern nint OpenProcess(
        uint processAccess,
        bool bInheritHandle,
        int processId
    );

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern nint OpenThread(
uint threadAccess,
bool bInheritHandle,
int threadId
);

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern bool CloseHandle(nint handle);

        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        public static extern nint CreateEvent(
IntPtr lpEventAttributes,
bool bManualReset,
bool bInitialState,
string lpName);

        [DllImport("kernel32.dll", SetLastError = true)]
        public static extern bool TerminateProcess(IntPtr hProcess, uint uExitCode);

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
