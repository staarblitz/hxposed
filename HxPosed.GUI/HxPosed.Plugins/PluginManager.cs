using HxPosed.Plugins.Config;
using Microsoft.Win32;
using System;
using System.Collections.Concurrent;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO.Pipelines;

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

        public static async Task<Plugin> LoadFromConfig(PluginConfig config, CancellationToken cancellationToken)
        {
            using var client = new HttpClient();
            await Parallel.ForEachAsync(config.Downloads, cancellationToken, async (download, token) =>
            {
                var bytes = await client.GetStreamAsync(download.Url, cancellationToken);
                using var fs = new FileStream(Environment.ExpandEnvironmentVariables(download.SaveLocation), FileMode.OpenOrCreate, FileAccess.Write, FileShare.Read);
                await bytes.CopyToAsync(fs, cancellationToken);

                if (download.ShellExecuteAfter)
                {
                    Process.Start(new ProcessStartInfo
                    {
                        UseShellExecute = true,
                        FileName = download.SaveLocation
                    });
                }
            });

            return Plugin.New(config.Guid.HasValue ? config.Guid.Value : Guid.NewGuid(), config.Name,
                config.Description, config.Version, config.Url, config.Author, config.Icon, config.Path, config.Permissions);
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
