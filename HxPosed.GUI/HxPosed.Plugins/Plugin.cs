using HxPosed.Plugins.Permissions;
using Microsoft.Win32;
using System;

namespace HxPosed.Plugins
{
    public class Plugin
    {
        private Plugin() { }

        public required string Name { get; init; }
        public required string Description { get; init; }
        public required uint Version { get; init; }

        public required string Url { get; init; }
        public required string Author { get; init; }
        public required string Icon { get; init; }

        private PluginStatus _status;
        public PluginStatus Status
        {
            get => _status; set
            {
                SetStatus(value, Error, Permissions);
                _status = value;
            }
        }
        public PluginError Error { get; protected set; }

        private PluginPermissions _permissions;
        public PluginPermissions Permissions
        {
            get => _permissions; set
            {
                SetStatus(Status, Error, value);
                _permissions = value;
            }
        }

        public Guid Guid { get; protected set; }

        /// <summary>
        /// Creates a new plugin in system registry, returns the plugin object.
        /// </summary>
        /// <param name="guid">Unique identifier of the plugin.</param>
        /// <param name="name">Name of the plugin.</param>
        /// <param name="description">Description of the plugin.</param>
        /// <param name="version">Version of the plugin.</param>
        /// <param name="url">Url of the plugin.</param>
        /// <param name="author">Author of the plugin.</param>
        /// <param name="icon">Icon of the plugin.</param>
        /// <returns>New instance of <see cref="Plugin"/></returns>
        /// <exception cref="ArgumentNullException">Throws if OpenSubKey returns null.</exception>
        public static Plugin New(Guid guid, string name, string description, uint version, string url, string author, string icon)
        {
            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins", true);
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            var plugin = new Plugin
            {
                Guid = guid,
                Name = name,
                Description = description,
                Version = version,
                Url = url,
                Author = author,
                Icon = icon,
                Error = PluginError.None,

                // Use the private fields since the setters call SetStatus when the registry key isn't prepared yet.
                _status = PluginStatus.Ready,
                _permissions = PluginPermissions.None,
            };

            using var pluginKey = key.CreateSubKey(guid.ToString());
            pluginKey.SetValue("Guid", guid.ToByteArray(), RegistryValueKind.Binary);
            pluginKey.SetValue("Name", name, RegistryValueKind.String);
            pluginKey.SetValue("Description", description, RegistryValueKind.String);
            pluginKey.SetValue("Version", version, RegistryValueKind.DWord);
            pluginKey.SetValue("URL", url, RegistryValueKind.String);
            pluginKey.SetValue("Author", author, RegistryValueKind.String);
            pluginKey.SetValue("Icon", icon, RegistryValueKind.String);

            pluginKey.SetValue("Error", (uint)PluginError.None, RegistryValueKind.DWord);
            pluginKey.SetValue("Status", (uint)PluginStatus.Ready, RegistryValueKind.DWord);
            pluginKey.SetValue("Permissions", (uint)PluginPermissions.None, RegistryValueKind.QWord);

            return plugin;
        }

        /// <summary>
        /// Loads the plugin from system registry.
        /// </summary>
        /// <param name="guid">Unique identifier of the plugin.</param>
        /// <returns>Reference to <see cref="Plugin"/></returns>
        /// <exception cref="ArgumentNullException">Throws if OpenSubKey returns null.</exception>
        public static Plugin Load(Guid guid)
        {
            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins\\{guid}");
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            return new Plugin
            {
                Guid = guid,
                Name = key.GetValue("Name").ToString(),
                Description = key.GetValue("Description").ToString(),
                Version = uint.Parse(key.GetValue("Version").ToString()),
                Url = key.GetValue("URL").ToString(),
                Author = key.GetValue("Author").ToString(),
                Icon = key.GetValue("Icon").ToString(),
                _status = (PluginStatus)(uint.Parse(key.GetValue("Status").ToString())),
                Error = (PluginError)(uint.Parse(key.GetValue("Error").ToString())),
                _permissions = (PluginPermissions)(ulong.Parse(key.GetValue("Permissions").ToString()))
            };
        }

        /// <summary>
        /// Removes plugin from system registry.
        /// WARNING! References to plugin object persists!
        /// </summary>
        /// <exception cref="ArgumentNullException">Throws if OpenSubKey returns null</exception>
        public void Remove()
        {
            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins", true);
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            key.DeleteSubKeyTree($"{Guid}", false);
        }

        /// <summary>
        /// Sets the status of plugin object, saves it into registry.
        /// Note: This is the function <see cref="Plugin.Status"/> and <see cref="Plugin.Permissions"/> setters internally call.
        /// </summary>
        /// <param name="status">Current status of the plugin.</param>
        /// <param name="error">Error of the plugin, if any.</param>
        /// <param name="permissions">Permissions of the plugin, if any.</param>
        /// <exception cref="ArgumentNullException">Throws if OpenSubKey returns null.</exception>
        public void SetStatus(PluginStatus status, PluginError error, PluginPermissions permissions)
        {
            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins\\{Guid}", true);
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            key.SetValue("Error", (uint)error, RegistryValueKind.DWord);
            key.SetValue("Status", (uint)status, RegistryValueKind.DWord);
            key.SetValue("Permissions", (uint)permissions, RegistryValueKind.QWord);
        }
    }
}
