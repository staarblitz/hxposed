using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Types
{
    public enum ServiceFunction
    {
        GetState = 0x0,
        OpenProcess = 0x10,
        CloseProcess = 0x11,
        GetProcessField = 0x12,
        SetProcessField = 0x13,
        RegisterNotifyEvent = 0x20,
        UnregisterNotifyEvent = 0x21,
        AllocateMemory = 0x30,
        FreeMemory = 0x31,
        GetSetPageAttribute = 0x32,
        MapRawMemoryDescriptor = 0x33,
        TranslateAddress = 0x34,
        DescribeMemory = 0x35,
        OpenThread = 0x40,
        CloseThread = 0x41,
        GetThreadField = 0x42,
        SetThreadField = 0x43,
        OpenToken = 0x50,
        CloseToken = 0x51,
        GetTokenField = 0x53,
        SetTokenField = 0x54,
        MsrIo = 0x60,
        ExecutePrivilegedInstruction = 0x61,
        InterProcessorInterrupt = 0x62,
        UpgradeHandle = 0x70,
        GetHandleObject = 0x71,
        SwapHandleObject = 0x72,
    }
}
