using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.PInvoke
{
    public static class Win32
    {
        private const int TOKEN_ADJUST_PRIVILEGES = 0x20;
        private const int TOKEN_QUERY = 0x8;
        private const int SE_PRIVILEGE_ENABLED = 0x2;

        [StructLayout(LayoutKind.Sequential)]
        struct LUID
        {
            public uint LowPart;
            public int HighPart;
        }

        [StructLayout(LayoutKind.Sequential)]
        struct TOKEN_PRIVILEGES
        {
            public int PrivilegeCount;
            public LUID Luid;
            public int Attributes;
        }

        [DllImport("advapi32.dll", SetLastError = true)]
        static extern bool OpenProcessToken(
            IntPtr ProcessHandle,
            int DesiredAccess,
            out IntPtr TokenHandle);

        [DllImport("advapi32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        static extern bool LookupPrivilegeValue(
            string lpSystemName,
            string lpName,
            out LUID lpLuid);

        [DllImport("advapi32.dll", SetLastError = true)]
        static extern bool AdjustTokenPrivileges(
            IntPtr TokenHandle,
            bool DisableAllPrivileges,
            ref TOKEN_PRIVILEGES NewState,
            int BufferLength,
            IntPtr PreviousState,
            IntPtr ReturnLength);

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
        public static string DosPathToDevicePath(string dosPath)
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

        /// <summary>
        /// Required for using <see cref="Variables"/>
        /// </summary>
        /// <exception cref="Win32Exception">Any of the pinvokes failed.</exception>
        public static void EnableSeSystemEnvironmentPrivilege()
        {
            if (!OpenProcessToken(Process.GetCurrentProcess().Handle, TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, out var token))
                throw new Win32Exception(Marshal.GetLastWin32Error());

            if (!LookupPrivilegeValue(null,"SeSystemEnvironmentPrivilege", out var luid))
                throw new Win32Exception(Marshal.GetLastWin32Error());

            var tp = new TOKEN_PRIVILEGES
            {
                PrivilegeCount = 1,
                Luid = luid,
                Attributes = SE_PRIVILEGE_ENABLED
            };

            if (!AdjustTokenPrivileges(token, false, ref tp, 0, IntPtr.Zero, IntPtr.Zero))
                throw new Win32Exception(Marshal.GetLastWin32Error());
        }
    }
}
