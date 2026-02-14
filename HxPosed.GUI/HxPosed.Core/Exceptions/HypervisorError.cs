using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core.Exceptions
{
    [StructLayout(LayoutKind.Explicit)]
    public struct HypervisorError
    {
        [FieldOffset(0)]
        public ErrorSource Source;
        [FieldOffset(4)]
        public ushort Error;
        [FieldOffset(6)]
        public ushort Reason;

        public void ThrowIfError()
        {
            if (IsError())
                throw new HypervisorException(this);
        }
        public readonly bool IsError() => !(Source == ErrorSource.Hx && (ErrorCode)Error == ErrorCode.Ok);
    }


    public enum ErrorSource : uint
    {
        Nt = 0,
        Hv = 1,
        Hx = 2,
    }

    public enum ErrorCode : ushort
    {
        Unknown = 0,
        Ok = 1,
        NotAllowed = 2,
        NotLoaded = 3,
        NotFound = 4,
        InvalidParams = 5,
    }
}
