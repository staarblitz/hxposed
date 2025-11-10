using HxPosed.Plugins;
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

        public Plugin Plugin { get; init; }
        public SymbolRegular Icon { get; init; }
    }
}
