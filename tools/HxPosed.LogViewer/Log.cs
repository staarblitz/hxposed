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
        AcquireObject = 1,
        FreeObject = 2,
        QueryObject = 3,
        TrackObject = 4,
        NoHxInfo = 5,
        HyperCall = 6,
        HcDispatch = 7,
        CallError = 8,
        HyperResult = 9,
        DetrackObject = 10,
        ProcessorVirtualized = 11,
        FailedToMap = 12,
        DelayedStart = 13,
        HxPosedInit = 14,
        NtVersion = 15,
        WindowsVersion = 16,
        FailedToAllocate = 17,
        Exception = 18,
        Catastrophic = 19,
        IncrementRefCount = 20,
        DecrementRefCount = 21,
        IncrementHandleCount = 22,
        DecrementHandleCount = 23,
        Vmxon = 24,
        Panic = 25,
        WritingAsyncBuffer = 26,
        WrittenAsyncBuffer = 27,
        NtInfo = 28,
        BuildOffset = 29
    }
}
