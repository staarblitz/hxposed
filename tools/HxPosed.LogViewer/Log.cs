using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.LogViewer
{
    [StructLayout(LayoutKind.Explicit, Pack = 1)]
    public struct LogEntry
    {
        [FieldOffset(0)]
        public LogType LogType;
        [FieldOffset(8)]
        public uint Processor;
        [FieldOffset(16)]
        public ulong Timestamp;
        [FieldOffset(24)]
        public LogEventTag LogEvent;
        [FieldOffset(32)]
        public ulong Arg1;
        [FieldOffset(40)]
        public ulong Arg2;
        [FieldOffset(48)]
        public ulong Arg3;
        [FieldOffset(56)]
        public ulong Arg4;
    }

    public enum LogType : byte
    {
        Trace = 0,
        Info = 1,
        Warn = 2,
        Error = 3
    }

    public enum LogEventTag : ulong
    {
        None = 0,
        VmxExitReason = 1,
        RIP = 2,
        VCPU = 3,
        UnknownExitReason = 4,
        NoHxInfo = 5,
        HyperCall = 6,
        HcDispatch = 7,
        CallError = 8,
        HyperResult = 9,
        VirtualizingProcessor = 10,
        ProcessorVirtualized = 11,
        FailedToMap = 12,
        DelayedStart = 13,
        HxPosedInit = 14,
        NtVersion = 15,
        WindowsVersion = 16,
        FailedToAllocate = 17,
        Exception = 18,
        Catastrophic = 19,
        ProcessorReady = 20,
        LaunchingProcessor = 21,
        Vmclear = 22,
        Vmptrld = 23,
        Vmxon = 24,
        Panic = 25,
        WritingAsyncBuffer = 26,
        WrittenAsyncBuffer = 27,
        NtInfo = 28,
        BuildOffset = 29
    }
}
