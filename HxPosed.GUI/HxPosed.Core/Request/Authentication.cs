using HxPosed.Plugins.Permissions;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Request
{
    [StructLayout(LayoutKind.Sequential)]
    public struct AuthenticationRequest
    {
        public Guid Guid;
        public PluginPermissions Permissions;
    }
}
