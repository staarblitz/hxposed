using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Management;
using System.Reflection.Emit;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Uefi
{
    public class Partition
    {
        public static ManagementObject GetEfiPartition()
        {
            using var query = new ManagementObjectSearcher("SELECT * FROM Win32_Volume");
            query.Options.ReturnImmediately = true;
            var results = query.Get();
            foreach (ManagementObject volume in results)
            {
                if ((string)volume["FileSystem"] == "FAT32" &&
                    (bool)volume["SystemVolume"] == true)
                {
                    return volume;
                }
            }

            throw new DriveNotFoundException("Cannot find EFI partition");
        }

        public static void MountEfiPartition(string label)
        {
            var volume = GetEfiPartition();
            var result = (int)(uint)volume.InvokeMethod("AddMountPoint", [label]);
            if (result == 3)
            {
                // already mounted
            }
            else if (result != 0)
            {
                throw new Win32Exception(result);
            }
        }

        public static void DismountEfiPartition()
        {
            var volume = GetEfiPartition();

            var name = (string)volume["DeviceID"];

            // we can, of course use WHERE bla bla, but lets keep it as c# as possible.
            using var query = new ManagementObjectSearcher("SELECT * FROM Win32_MountPoint");
            query.Options.ReturnImmediately = true;
            var results = query.Get();
            foreach (ManagementObject point in results)
            {
                if ((string)point["Volume"] == volume.Path.RelativePath)
                {
                    point.Delete();
                }
            }

            throw new DriveNotFoundException("Could not find mounted EFI partition!");
        }
    }
}
