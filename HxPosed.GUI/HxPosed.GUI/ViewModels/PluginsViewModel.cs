using HxPosed.GUI.Models;
using HxPosed.Plugins;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Windows;
using Wpf.Ui.Controls;

namespace HxPosed.GUI.ViewModels
{
    internal class PluginsViewModel : INotifyPropertyChanged
    {
        public event PropertyChangedEventHandler? PropertyChanged;

        private bool _isLoading = true;
        public bool IsLoading
        {
            get => _isLoading;
            set
            {
                _isLoading = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(IsLoading)));
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(LoadingVisibility)));
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(MainVisibility)));
            }
        }

        public ObservableCollection<PluginModel> Plugins
        {
            get
            {
                return GetPlugins();
            }
        }

        public ObservableCollection<PluginModel> GetPlugins()
        {
            IsLoading = true;
            var list = new ObservableCollection<PluginModel>(
            PluginManager.Plugins.Select(plugin =>
            {
                 var icon = Enum.TryParse<SymbolRegular>(plugin.Icon, out var parsed)
                       ? parsed
                       : SymbolRegular.Apps24;

                    return new PluginModel
                    {
                        Icon = icon,
                        Plugin = plugin
                    };
                })
            );

            IsLoading = false;
            return list;
        }


        public string StatusText
        {
            get => $"Total of {PluginManager.Plugins.Count} plugins on system.";
        }

        public Visibility LoadingVisibility => IsLoading ? Visibility.Visible : Visibility.Collapsed;
        public Visibility MainVisibility => LoadingVisibility == Visibility.Visible ? Visibility.Collapsed : Visibility.Visible;
    }
}
