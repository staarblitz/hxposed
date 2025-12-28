using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.PInvoke
{
    public static class HxPosed
    {
        [DllImport("libhxposed.dll")]
        private static extern HxExResult HxpTrap();
    }
}
