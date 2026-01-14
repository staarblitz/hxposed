using HxPosed.Core.Guard;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.GUI.ViewModels
{
    internal class SettingsViewModel : INotifyPropertyChanged
    {
        public bool TamperProtectionEnabled
        {
           get { return HxGuard.RegistryProtection.GetRegistryProtection(); }
           set {HxGuard.RegistryProtection.SetRegistryProtection(value);
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(TamperProtectionEnabled))); }
        }

        public event PropertyChangedEventHandler? PropertyChanged;
    }
}
