using HxPosed.Plugins.Permissions;
using Microsoft.Win32;

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

        public PluginStatus Status { get; protected set; }
        public PluginError Error { get; protected set; }
        public PluginPermissions Permissions { get; protected set; }

        public Guid Guid { get; protected set; }


        public void SetStatus(PluginStatus status, PluginError error, PluginPermissions permissions)
        {
            using var key = Registry.LocalMachine.OpenSubKey($"Software\\HxPosed\\Plugins\\{Guid}", true);
            if (key is null)
            {
                throw new ArgumentNullException(nameof(key));
            }

            key.SetValue("Error", (uint)error, RegistryValueKind.DWord);
            key.SetValue("Status", (uint)status, RegistryValueKind.DWord);
            key.SetValue("Permissions", (uint)status, RegistryValueKind.QWord);
        }

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
                Status = PluginStatus.Ready,
                Error = PluginError.None
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
                Status = (PluginStatus)(uint.Parse(key.GetValue("Status").ToString())),
                Error = (PluginError)(uint.Parse(key.GetValue("Error").ToString())),
                Permissions = (PluginPermissions)(ulong.Parse(key.GetValue("Permissions").ToString()))
            };
        }
    }
}
