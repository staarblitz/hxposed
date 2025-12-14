using HxPosed.Core.Request;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Response
{
    // Evil c# union hack
    [StructLayout(LayoutKind.Explicit)]
    public struct GetProcessFieldResponse
    {
        [FieldOffset(0)]
        public ProcessField Field;
        [FieldOffset(4)]
        public ProcessProtection Protection;
        [FieldOffset(4)]
        public ProcessSigners Signers;
        [FieldOffset(4)]
        public ProcessMitigationFlags MitigationFlags;
        [FieldOffset(4)]
        public IntPtr Token;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct GetProcessThreadsResponse
    {
        public uint NumberOfThreads;
    }
}
