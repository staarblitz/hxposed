using Microsoft.Win32;
using System;
using System.Collections.Concurrent;
using System.Collections.ObjectModel;

namespace HxPosed.Plugins
{
    public class PluginManager
    {
        public static void HealthCheck()
        {
            using var key = Registry.LocalMachine.OpenSubKey("Software", true);
            RegistryKey mainKey = null;
            if (!key.GetSubKeyNames().Contains("HxPosed"))
                mainKey = key.CreateSubKey("HxPosed", true);
            else
                mainKey = key.OpenSubKey("HxPosed", true);

            if (!mainKey.GetSubKeyNames().Contains("Plugins"))
            {
                mainKey.CreateSubKey("Plugins").Dispose();
                Plugin.New(Guid.NewGuid(), "Test Plugin", "Showcases how the UI looks", 1, "https://github.com/Staarblitz", "Staarblitz", "App24");
            }
                

            mainKey.Dispose();
        }

        private static void GetPlugins()
        {
            _plugins.Clear();

            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins");
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            foreach (var subkey in key.GetSubKeyNames())
            {
                if(!Guid.TryParse(subkey, out var guid))
                {
                    // log
                    continue;
                }
                _plugins.Add(Plugin.Load(guid));
            }
        }

        private static ObservableCollection<Plugin> _plugins = [];
        public static ObservableCollection<Plugin> Plugins
        {
            get
            {
                GetPlugins();
                return _plugins;
            }
        }
    }
}
