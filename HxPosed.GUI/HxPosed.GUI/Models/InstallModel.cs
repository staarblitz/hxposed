using HxPosed.Core.Uefi;
using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Management;
using System.Runtime.InteropServices;
using System.Security.Principal;
using System.Text;
using System.Threading.Tasks;
using System.Windows;

namespace HxPosed.GUI.Models
{
    public class InstallModel
    {


        public static int GetWindowsBuildNumber()
        {
            using var key = Registry.LocalMachine.OpenSubKey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion");
            return int.Parse(key.GetValue("CurrentBuild").ToString());
        }

        public static bool GetVTxSupport()
        {
            return true;
        }

        public static bool GetSecureBootEnabled()
        {
            byte value = 0;
            var handle = GCHandle.Alloc(value, GCHandleType.Pinned);
            var ptr = handle.AddrOfPinnedObject();
            Variables.GetFirmwareEnvironmentVariable("SecureBoot", "{8be4df61-93ca-11d2-aa0d-00e098032b8c}", ptr, 1);

            handle.Free();

            return value == 1;
        }

        public static bool GetUefiBoot()
        {
            // returns ERROR_INVALID_FUNCTION (1) when UEFI is not present
            return Variables.GetFirmwareEnvironmentVariable("", "{00000000-0000-0000-0000-000000000000}", IntPtr.Zero, 0) != 1;
        }

        public static bool IsAdministrator =>
   new WindowsPrincipal(WindowsIdentity.GetCurrent())
       .IsInRole(WindowsBuiltInRole.Administrator);
    }
}
