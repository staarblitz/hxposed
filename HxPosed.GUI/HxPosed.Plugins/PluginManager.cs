using Microsoft.Win32;
using System;
using System.Collections.Concurrent;
using System.Collections.ObjectModel;

namespace HxPosed.Plugins
{
    public class PluginManager
    {
        /// <summary>
        /// Checks the sanity of registry keys for plugin management.
        /// Adds a default plugin if doesn't exists in DEBUG mode.
        /// </summary>
        public static void HealthCheck()
        {
            try
            {
                using var key = Registry.LocalMachine.OpenSubKey("Software", true);
                RegistryKey mainKey = null;
                if (!key!.GetSubKeyNames().Contains("HxPosed"))
                    mainKey = key.CreateSubKey("HxPosed", true);
                else
                    mainKey = key.OpenSubKey("HxPosed", true)!;

                if (!mainKey.GetSubKeyNames().Contains("Plugins"))
                {
                    mainKey.CreateSubKey("Plugins").Dispose();
#if DEBUG
                    Plugin.New(Guid.NewGuid(), "Test Plugin", "Showcases how the UI looks", 1, "https://github.com/Staarblitz", "Staarblitz", "App24");
#endif
                }


                mainKey.Dispose();
            }
            catch
            {

            }
        }

        /// <summary>
        /// Gets the plugins, saves them to global _plugins collection.
        /// </summary>
        /// <exception cref="ArgumentNullException">Throws if OpenSubKey returns null</exception>
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
