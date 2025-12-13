using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core.Exceptions
{
    [StructLayout(LayoutKind.Sequential, Pack =1)]
    internal struct HypervisorError
    {
        public ErrorSource Source;
        public ushort Error;
        public ushort Reason;

        public readonly bool IsError() => !(Source == ErrorSource.Hx && (ErrorCode)Error == ErrorCode.Ok);
    }


    internal enum ErrorSource : ushort
    {
        Nt = 0,
        Hv = 1,
        Hx = 2,
    }

    internal enum ErrorCode: ushort
    {
        Unknown = 0,
        Ok = 1,
        NotAllowed = 2,
        NotLoaded = 3,
        NotFound = 4,
        InvalidParams = 5,
    }
}
