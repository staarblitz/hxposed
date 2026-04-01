using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core.Exceptions
{
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public partial struct HxResult
    {
        public _HX_ERROR_CODE ErrorCode;
        public _Anonymous_e__Union Anonymous;

        public void ThrowIfError()
        {
            if (ErrorCode != 0)
            {
                var errorText = ErrorCode switch
                {
                    _HX_ERROR_CODE.HxErrSuccess => "Success",
                    _HX_ERROR_CODE.HxErrNotAllowed => "Not allowed",
                    _HX_ERROR_CODE.HxErrNotFound => $"Object {Anonymous.NotFoundReason} not found",
                    _HX_ERROR_CODE.HxErrInvalidParameters => $"Invalid parameter {Anonymous.Parameter} passed",
                    _HX_ERROR_CODE.HxErrNtError => $"NT error {Anonymous.NtStatus}",
                    _HX_ERROR_CODE.HxErrHxNotLoaded => "HxPosed is not loaded",
                    _HX_ERROR_CODE.HxErrTimedOut => "Timeout"
                };
                throw new Exception(errorText);
            }
        }

        public ref _HX_NOT_ALLOWED_REASON NotAllowedReason
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NotAllowedReason, 1));
            }
        }

        public ref _HX_NOT_FOUND_REASON NotFoundReason
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NotFoundReason, 1));
            }
        }

        public ref uint NtStatus
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.NtStatus, 1));
            }
        }

        public ref uint Parameter
        {
            get
            {
                return ref MemoryMarshal.GetReference(MemoryMarshal.CreateSpan(ref Anonymous.Parameter, 1));
            }
        }

        [StructLayout(LayoutKind.Explicit, Pack = 1)]
        public partial struct _Anonymous_e__Union
        {
            [FieldOffset(0)]
            public _HX_NOT_ALLOWED_REASON NotAllowedReason;

            [FieldOffset(0)]
            public _HX_NOT_FOUND_REASON NotFoundReason;

            [FieldOffset(0)]
            public uint NtStatus;

            [FieldOffset(0)]
            public uint Parameter;
        }
    }

    public enum _HX_ERROR_CODE
    {
        HxErrSuccess = 0,
        HxErrNotAllowed = 1,
        HxErrNotFound = 2,
        HxErrInvalidParameters = 3,
        HxErrNtError = 4,
        HxErrTimedOut = 5,
        HxErrHxNotLoaded = 6,
    }

    public enum _HX_NOT_ALLOWED_REASON
    {
       LockHeld = 2,
       PageNotPresent = 3,
       MappingsExist = 4,
       AccessViolation = 5,
    }

    public enum _HX_NOT_FOUND_REASON
    {
       Process = 1,
       Mdl = 3,
       Thread = 4,
       Function = 5,
       Token = 6,
       Callback = 7,
       Event = 9,
       Field = 10,
       Handle = 11,
    }
}
