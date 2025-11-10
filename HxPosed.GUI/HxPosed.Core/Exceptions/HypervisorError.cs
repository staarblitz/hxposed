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
        public ErrorCode Error;

        public readonly bool IsError() => !(Source == ErrorSource.Hx && Error == ErrorCode.Ok);
    }


    internal enum ErrorSource : uint
    {
        Nt = 0,
        Hv = 1,
        Hx = 2,
    }

    internal enum ErrorCode: uint
    {
        Unknown = 0,
        Ok = 1,
        NotAllowed = 2,
        NotLoaded = 3
    }
}
