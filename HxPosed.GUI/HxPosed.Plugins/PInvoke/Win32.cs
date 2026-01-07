using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Plugins.PInvoke
{
    internal static class Win32
    {
        [DllImport("kernel32.dll", CharSet = CharSet.Unicode)]
        private static extern int QueryDosDevice(
         string lpDeviceName,
         StringBuilder lpTargetPath,
         int ucchMax
        );

        /// <summary>
        /// Converts a conventional DOS path (e.g. "C:\\Windows\\regedit.exe") to NT style device path (e.g. \\Device\\HarddiskVolume3\\Windows\\regedit.exe)
        /// This is required because kernel drivers don't use DOS paths.
        /// </summary>
        /// <param name="dosPath">DOS path tto convert.</param>
        /// <returns>NT style path</returns>
        /// <exception cref="ArgumentException">DOS path is invalid.</exception>
        /// <exception cref="Win32Exception">Something went wrong executing QueryDosDevice.</exception>
        internal static string DosPathToDevicePath(string dosPath)
        {
            if (string.IsNullOrEmpty(dosPath) || dosPath.Length < 2 || dosPath[1] != ':')
                throw new ArgumentException("Invalid DOS path");

            string drive = dosPath.Substring(0, 2);
            string rest = dosPath.Substring(2);

            var sb = new StringBuilder(260);
            int result = QueryDosDevice(drive, sb, sb.Capacity);
            if (result == 0)
                throw new Win32Exception(result);

            return sb.ToString() + rest;
        }
    }
}
