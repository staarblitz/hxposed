using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Uefi
{
    public class BootOrder
    {
        /// <summary>
        /// Sets boot order. The first item in the enumerable is the first item to be shown.
        /// </summary>
        /// <param name="newOrder">New order</param>
        /// <exception cref="Win32Exception"><see cref="Variables.SetFirmwareEnvironmentVariableEx(string, string, nint, uint, uint)" Fails/></exception>
        public static void SetBootOrder(IEnumerable<ushort> newOrder)
        {
            using var stream = new MemoryStream();
            using var writer = new BinaryWriter(stream, Encoding.Unicode);

            foreach (var item in newOrder)
            {
                writer.Write(item);
            }

            var buffer = stream.ToArray();

            unsafe
            {
                fixed (byte* ptr = buffer)
                {
                    if (!Variables.SetFirmwareEnvironmentVariableEx("BootOrder", Variables.EFI_GLOBAL_GUID, (nint)ptr, (uint)buffer.Length,
                        Variables.VARIABLE_ATTRIBUTE_RUNTIME_ACCESS | Variables.VARIABLE_ATTRIBUTE_BOOTSERVICE_ACCESS | Variables.VARIABLE_ATTRIBUTE_NON_VOLATILE))
                    {
                        throw new Win32Exception(Marshal.GetLastWin32Error());
                    }
                }
            }
        }

        /// <summary>
        /// Gets the boot order. First item is first to be shown.
        /// </summary>
        /// <returns>A <see cref="List{ushort}"/> that contains the current boot order.</returns>
        /// <exception cref="Win32Exception"><see cref="Variables.GetFirmwareEnvironmentVariable(string, string, nint, uint)"/> Fails</exception>
        public static List<ushort> GetBootOrder()
        {
            // shall be more than enough
            var alloc = Marshal.AllocHGlobal(1024);
            var result = Variables.GetFirmwareEnvironmentVariable("BootOrder", Variables.EFI_GLOBAL_GUID, alloc, 1024);
            if (result == 0)
                throw new Win32Exception(result);

            var list = new List<ushort>();

            UnmanagedMemoryStream stream;
            unsafe
            {
                stream = new UnmanagedMemoryStream((byte*)alloc.ToPointer(), result);
            }
            using var reader = new BinaryReader(stream, Encoding.Unicode);

            try
            {
                while(stream.Position <= result)
                {
                    list.Add(reader.ReadUInt16());
                }
            }
            catch (Exception ex)
            {
                if (ex is not EndOfStreamException)
                    throw;
            }
            finally
            {
                stream.Dispose();
            }

            return list;
        }
    }
}
