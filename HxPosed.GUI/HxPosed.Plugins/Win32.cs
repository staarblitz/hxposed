using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Plugins
{
    internal static class Win32
    {
        [DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
        private static extern uint QueryDosDevice(
         string lpDeviceName,
         StringBuilder lpTargetPath,
         int ucchMax
        );

        internal static string DosPathToDevicePath(string dosPath)
        {
            if (string.IsNullOrEmpty(dosPath) || dosPath.Length < 2 || dosPath[1] != ':')
                throw new ArgumentException("Invalid DOS path");

            string drive = dosPath.Substring(0, 2);
            string rest = dosPath.Substring(2);

            var sb = new StringBuilder(260);
            uint result = QueryDosDevice(drive, sb, sb.Capacity);
            if (result == 0)
                throw new Win32Exception();

            return sb.ToString() + rest;
        }
    }
}
