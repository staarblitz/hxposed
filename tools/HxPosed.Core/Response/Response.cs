using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Response
{
    [StructLayout(LayoutKind.Sequential)]
    public struct OpenObjectResponse
    {
        public IntPtr Address;
    }
}
