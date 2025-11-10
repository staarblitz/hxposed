using HxPosed.GUI.ViewModels;
using HxPosed.Plugins;
using HxPosed.Plugins.Permissions;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Text;
using Wpf.Ui.Controls;

namespace HxPosed.GUI.Models
{
    internal class PluginModel : INotifyPropertyChanged
    {
        public event PropertyChangedEventHandler? PropertyChanged;
        public IEnumerable<PermissionEntry> PermissionEntries => Enum.GetValues<PluginPermissions>()
    .Where(v => v != PluginPermissions.None && !(v.ToString().StartsWith("Reserved")))
    .Select(v => new PermissionEntry(v, this));

        public required Plugin Plugin { get; init; }
        public SymbolRegular Icon { get; init; }
    }
}
