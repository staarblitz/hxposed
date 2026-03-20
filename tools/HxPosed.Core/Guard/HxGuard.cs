using HxPosed.Core.Cryptography;
using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Guard
{
    public class HxGuard
    {
        private static RegistryKey _reg = Registry.LocalMachine.OpenSubKey("SOFTWARE\\HxPosed\\HxGuard", true);

        public static RegistryProtectionSettings RegistryProtection { get; } = new();
        public static CallerVerificationSettings CallerVerification { get; } = new();

        public class RegistryProtectionSettings
        {
            public void SetRegistryProtection(bool status)
            {
                _reg.SetValue("RegistryProtection", status ? 1 : 0, RegistryValueKind.DWord);
            }

            public bool GetRegistryProtection()
            {
                var val = _reg.GetValue("RegistryProtection");
                if (val == null)
                {
                    SetRegistryProtection(true);
                    return true;
                }
                return (int)val == 1;
            }
        }

       

        public class CallerVerificationSettings
        {
            public record class VerifiedCaller
            {
                public static VerifiedCaller FromFilePath(string filePath)
                {
                    return new VerifiedCaller
                    {
                        FilePath = filePath,
                        PathHash = WyHash64.ComputeHash64(filePath, 0x2009)
                    };
                }

                public required string FilePath { get; set; }
                public required ulong PathHash { get; set; }
            }

            private static RegistryKey _optionsKey = _reg.OpenSubKey("CallerVerification", true);
            public void SetCallerVerification(bool status)
            {
                _reg.SetValue("CallerVerification", status ? 1 : 0, RegistryValueKind.DWord);
            }

            public bool GetCallerVerification()
            {
                var val = _reg.GetValue("CallerVerification");
                if (val == null)
                {
                    SetCallerVerification(true);
                    return true;
                }
                return (int)val == 1;
            }

            public List<VerifiedCaller> GetVerifiedCallers()
            {
                var list = new List<VerifiedCaller>(256);
                foreach(var value in _optionsKey.GetValueNames())
                {
                    if (value == "VerifiedCallers") continue;

                    list.Add(new VerifiedCaller
                    {
                        FilePath = value,
                        PathHash = (ulong)(long)(_optionsKey.GetValue(value)!)
                    });
                }

                return list;
            }

            public void SetVerifiedCallers(List<VerifiedCaller> callers)
            {
                // clear
                foreach(var value in _optionsKey.GetValueNames())
                {
                    if (value == "VerifiedCallers") continue;
                    _optionsKey.DeleteValue(value);
                }

                if(callers.Count == 0)
                {
                    _optionsKey.SetValue("VerifiedCallers", new byte[256 * 8], RegistryValueKind.Binary);
                    return;
                }

                var hashes = callers.Select(x => Cryptography.WyHash64.ComputeHash64(x.FilePath, 0x2009)).ToArray();
                using var bytes = new MemoryStream(256 * 8);
                using var writer = new BinaryWriter(bytes);

                for (int i = 0; i < callers.Count; i++)
                {
                    // cast to long, because SetValue has a bug. it treats QWord as an i64, not u64.
                    _optionsKey.SetValue(callers[i].FilePath, (long)hashes[i], RegistryValueKind.QWord);
                    writer.Write(hashes[i]);
                }

                if (bytes.Position > 256 * 8)
                {
                    throw new ArgumentOutOfRangeException("Too many entries");
                }

                // not ToArray, because we need the full array in its const size.
                _optionsKey.SetValue("VerifiedCallers", bytes.GetBuffer(), RegistryValueKind.Binary);
            }
        }
    }
}
