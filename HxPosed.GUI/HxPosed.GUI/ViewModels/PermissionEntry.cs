using HxPosed.GUI.Models;
using HxPosed.Plugins.Permissions;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.ComponentModel.DataAnnotations;
using System.Reflection;
using System.Text;

namespace HxPosed.GUI.ViewModels
{
    internal class PermissionEntry : INotifyPropertyChanged
    {
        private readonly PluginModel _plugin;

        public event PropertyChangedEventHandler? PropertyChanged;

        public PluginPermissions Permission { get; }

        public string Header { get; }
        public string Subtitle { get; }

        public bool IsEnabled
        {
            get => _plugin.Plugin.Permissions.HasFlag(Permission);
            set
            {
                if (value)
                    _plugin.Plugin.Permissions |= Permission;
                else
                    _plugin.Plugin.Permissions &= ~Permission;

                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(IsEnabled)));
            }
        }

        internal PermissionEntry(PluginPermissions permission, PluginModel plugin)
        {
            var field = typeof(PluginPermissions).GetField(permission.ToString());
            var display = field?.GetCustomAttribute<DisplayAttribute>();

            Permission = permission;
            Header = display?.Name ?? permission.ToString();
            Subtitle = display?.Description ?? $"Allows {permission} access";

            _plugin = plugin;
        }
    }

}
