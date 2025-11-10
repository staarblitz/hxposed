using System;
using System.Collections.Generic;
using System.Text;

namespace HxPosed.Plugins.Permissions
{
    [Flags]
    public enum PluginPermissions : ulong
    {
        None = 0,
        ProcessExecutive = 1 << 0,
        ProcessMemory = 1 << 1,
        ProcessProtection = 1 << 2,
        Reserved = 1 << 3,

        Reserved2 = 1 << 4,
        Reserved3 = 1 << 5,
        Reserved4 = 1 << 6,
        Reserved5 = 1 << 7,

        MemoryVirtual = 1 << 8,
        MemoryPhysical = 1 << 9,
        MemoryAllocation = 1 << 10,
        MemoryProtect = 1 << 11,
        MemoryIsolate = 1 << 12,

        Reserved6 = 1 << 13,
        Reserved7 = 1 << 14,
        Reserved8 = 1 << 15,
        Reserved9 = 1 << 16,
        Reserved10 = 1 << 17,

        CpuMsrRead = 1 << 18,
        CpuMsrWrite = 1 << 19,
        CpuSegmentation = 1 << 20,
        CpuControl = 1 << 21,

        Reserved11 = 1 << 22,
        Reserved12 = 1 << 23,
        Reserved13 = 1 << 24,
        Reserved14 = 1 << 25,
        Reserved15 = 1 << 26,

        SecurityNew = 1 << 27,
        SecurityManage = 1 << 28,
        SecurityDelete = 1 << 29,

        // Rest is reserved
    }
}
