using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Uefi
{
    public static class Variables
    {
        public const string EFI_GLOBAL_GUID = "{8be4df61-93ca-11d2-aa0d-00e098032b8c}";

        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        public static extern int GetFirmwareEnvironmentVariable(string lpName, string lpGUID, IntPtr pBuffer, uint size);

        [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        public static extern bool SetFirmwareEnvironmentVariableEx(string lpName, string lpGUID, IntPtr pBuffer, uint size, uint attributes);

        public const uint VARIABLE_ATTRIBUTE_NON_VOLATILE = 0x1;
        public const uint VARIABLE_ATTRIBUTE_BOOTSERVICE_ACCESS = 0x2;
        public const uint VARIABLE_ATTRIBUTE_RUNTIME_ACCESS = 0x4;
    }
}
