using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core.Response
{
    [StructLayout(LayoutKind.Sequential)]
    public struct StatusResponse
    {
        public HypervisorStatus Status;
        public int Version;
    }
}
