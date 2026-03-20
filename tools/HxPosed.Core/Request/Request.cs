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
}
