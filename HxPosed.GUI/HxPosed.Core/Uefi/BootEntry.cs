using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Uefi
{
    public class BootEntry
    {
        private BootEntry() { }

        public EfiLoadOptionAttributes Attributes { get; private set; }
        public string Name { get; private set; }
        public string Description { get; private set; }
        private ushort _filePathListLength;
        public List<EfiDevicePathProtocol> ProtocolList { get; private set; } = [];

        /// <summary>
        /// Creates a new boot entry. To add it into boot order, see <see cref="BootOrder.SetBootOrder(IEnumerable{ushort})"/>
        /// </summary>
        /// <param name="name">Name of the order. Boot####</param>
        /// <param name="description">The text that pops up in the boot menu</param>
        /// <param name="attributes">Attributes of the order.</param>
        /// <param name="protocolList">Protocol list describing where the file to be loaded is at.</param>
        /// <exception cref="Win32Exception"><see cref="Variables.SetFirmwareEnvironmentVariableEx(string, string, nint, uint, uint)"/> failed</exception>
        public static void NewEntry(string name, string description, EfiLoadOptionAttributes attributes, IEnumerable<EfiDevicePathProtocol> protocolList)
        {
            using var stream = new MemoryStream(1024);
            using var writer = new BinaryWriter(stream, Encoding.Unicode);

            writer.Write((int)attributes);

            var list = protocolList.ToList();

            ushort pathBytes = 0;
            foreach (var p in list)
                pathBytes += (ushort)(4 + p.Data.Length);

            // end of path whatever.
            pathBytes += 4;

            writer.Write(pathBytes);

            writer.Write(description.ToCharArray());
            writer.Write((ushort)0);

            foreach (var p in list)
            {
                ushort len = (ushort)(4 + p.Data.Length);
                writer.Write((byte)p.Type);
                writer.Write((byte)p.SubType);
                writer.Write(len);
                writer.Write(p.Data);
            }

            writer.Write((byte)0x7F); // end
            writer.Write((byte)0xFF); // end entire
            writer.Write((ushort)4);  // len 4 bytes

            var buf = stream.ToArray();
            unsafe
            {
                fixed(byte* p = buf)
                {
                    if (!Variables.SetFirmwareEnvironmentVariableEx(name, Variables.EFI_GLOBAL_GUID, (nint)p, (uint)buf.Length,
                        Variables.VARIABLE_ATTRIBUTE_RUNTIME_ACCESS | Variables.VARIABLE_ATTRIBUTE_BOOTSERVICE_ACCESS | Variables.VARIABLE_ATTRIBUTE_NON_VOLATILE))
                    {
                        throw new Win32Exception(Marshal.GetLastWin32Error());
                    }
                }
            }
        }

        /// <summary>
        /// Reads a boot entry with the name.
        /// </summary>
        /// <param name="name">Name of the boot entry</param>
        /// <returns><see cref="BootEntry"/> containing information about the entry.</returns>
        /// <exception cref="Win32Exception"><see cref="Variables.GetFirmwareEnvironmentVariable(string, string, nint, uint)"/> fails</exception>
        public static BootEntry ReadEntry(string name)
        {
            var buf = Marshal.AllocHGlobal(1024);
            var result = Variables.GetFirmwareEnvironmentVariable(name, Variables.EFI_GLOBAL_GUID, buf, 1024);
            if (result == 0)
            {
                Marshal.FreeHGlobal(buf);
                throw new Win32Exception(Marshal.GetLastWin32Error());
            }

            var me = new BootEntry
            {
                Name = name
            };

            UnmanagedMemoryStream stream;
            unsafe
            {
                stream = new UnmanagedMemoryStream((byte*)buf.ToPointer(), result);
            }
            using var reader = new BinaryReader(stream, Encoding.Unicode);

            me.Attributes = (EfiLoadOptionAttributes)reader.ReadInt32();
            me._filePathListLength = reader.ReadUInt16();
            
            var builder = new StringBuilder();

            while (true)
            {
                var c = reader.ReadChar();
                if (c == '\0')
                    break;
                builder.Append(c);
            }

            me.Description = builder.ToString();

            var bytesRead = 0;

            while (bytesRead < me._filePathListLength)
            {
                var type = reader.ReadByte();
                var subType = reader.ReadByte();
                var length = reader.ReadUInt16();

                bytesRead += length;

                var dataLength = length - 4;
                var data = reader.ReadBytes(dataLength);

                var protocol = new EfiDevicePathProtocol
                {
                    Type = (EfiDevicePathProtocolType)type,
                    SubType = (EfiDevicePathProtocolSubType)subType,
                    Length = length,
                    Data = data,
                };

                me.ProtocolList.Add(protocol);

                // ok, we are done
                if (type == 0x7F && subType == 0xFF)
                    break;
            }

            Marshal.FreeHGlobal(buf);
            stream.Dispose();

            return me;
        }
    }

    // https://uefi.org/specs/UEFI/2.10/10_Protocols_Device_Path_Protocol.html
    public record class EfiDevicePathProtocol
    {
        public EfiDevicePathProtocol() { }

        private EfiDevicePathProtocol(EfiDevicePathProtocolType type, EfiDevicePathProtocolSubType subType, byte[] data)
        {
            Type = type;
            SubType = subType;
            Data = data;
            Length = (ushort)Data.Length;
        }

        public EfiDevicePathProtocolType Type;
        public EfiDevicePathProtocolSubType SubType;
        public ushort Length;
        public byte[] Data;

        public static EfiDevicePathProtocol FromDevicePath(string path)
        {
            return new EfiDevicePathProtocol(EfiDevicePathProtocolType.MediaDevicePath, EfiDevicePathProtocolSubType.VendorDevicePath, Encoding.Unicode.GetBytes(path + '\0'));
        }

        public string GetDevicePath()
        {
            return Encoding.Unicode.GetString(Data).TrimEnd('\0');
        }
    }

    // aint making new files just for a few enums

    public enum EfiDevicePathProtocolType : byte
    {
        HardwareDevicePath = 0x1,
        ACPIDevicePath = 0x2,
        MessagingDevicePath = 0x3,
        MediaDevicePath = 0x4,
        BIOSBootSpecificationDevicePath = 0x5,
        End = 0x7F
    }

    public enum EfiDevicePathProtocolSubType : byte
    {
        EndThisInstance = 0x1,
        HardDrive = 0x1,
        PCI = 0x1,
        PCCARD = 0x2,
        MemoryMapped = 0x3,
        VendorDevicePath = 0x4,
        Controller = 0x5,
        BMC = 0x6,
        EndEntire = 0xFF
    }

    public enum EfiLoadOptionAttributes : uint
    {
        None = 0,
        Active = 1 << 0,
        ForceReconnect = 1 << 1,
        LoadOptionHidden = 1 << 4,
    }
}
