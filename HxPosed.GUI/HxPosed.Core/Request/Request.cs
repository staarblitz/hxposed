using HxPosed.Core.Exceptions;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Request
{
    public enum ObjectOpenType : uint
    {
        Handle = 0,
        Hypervisor = 1
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct AsyncInfo
    {
        public IntPtr AsyncHandle;
        public ulong Result;
        public ulong Arg1;
        public ulong Arg2;
        public ulong Arg3;

        public static AsyncInfo Initialize()
        {
            return new AsyncInfo
            {
                Arg1 = 0,
                Arg2 = 0,
                Arg3 = 0,
                Result = 0,
                AsyncHandle = HxpCreateEventHandle()
            };
        }

        [DllImport("libhxposed.dll")]
        private static extern IntPtr HxpCreateEventHandle();

        [DllImport("libhxposed.dll")]
        public static extern HypervisorError HxpResponseFromAsync(ref AsyncInfo async, ref IntPtr result);
    }

    public class HxWaitHandle : WaitHandle
    {

    }
}
