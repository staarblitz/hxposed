using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Types
{
    public struct HxStatus
    {
        public HxRuntimeStatus Status;
        public uint _PAD;
        public uint Version;
        public uint _PAD2;
    }

    public enum HxRuntimeStatus
    {
        Unknown = 0,
        SystemVirtualized = 1,
        SystemDeVirtualized = 2,
    }
}
