using HxPosed.Core;
using HxPosed.Core.Response;
using HxPosed.GUI.Models;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Navigation;
using Wpf.Ui.Controls;

namespace HxPosed.GUI.ViewModels
{
    public class InstallViewModel : INotifyPropertyChanged
    {
        // yes i do use the type itself instead of some weird ass converter logic
        // wanna fight about it?
        public InfoBarSeverity UefiSeverity {
            get
            {
                return InstallModel.GetUefiBoot() switch
                {
                    true => InfoBarSeverity.Success,
                    false => InfoBarSeverity.Error,
                };
            }
        }

        public InfoBarSeverity SecureBootSeverity
        {
            get
            {
                return InstallModel.GetSecureBootEnabled() switch
                {
                    false => InfoBarSeverity.Success,
                    true => InfoBarSeverity.Warning,
                };
            }
        }

        public InfoBarSeverity AdminPrivilegesSeverity
        {
            get
            {
                return InstallModel.IsAdministrator switch
                {
                    true => InfoBarSeverity.Success,
                    false => InfoBarSeverity.Warning,
                };
            }
        }
        public InfoBarSeverity VTxSeverity
        {
            get
            {
                return InstallModel.GetVTxSupport() switch
                {
                    true => InfoBarSeverity.Success,
                    false => InfoBarSeverity.Error,
                };
            }
        }

        public InfoBarSeverity WindowsSeverity
        {
            get
            {
                if(InstallModel.GetWindowsBuildNumber() == 26200)
                    return InfoBarSeverity.Success;
                return InfoBarSeverity.Error;
            }
        }

        private bool _canInstall;
        public bool CanInstall
        {
            get => _canInstall;
            set
            {
                _canInstall = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(CanInstall)));
            }
        }

        private Visibility _progressBarVisibility = Visibility.Hidden;
        public Visibility ProgressBarVisibility
        {
            get => _progressBarVisibility;
            set
            {
                _progressBarVisibility = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(ProgressBarVisibility)));
            }
        }

        private string _statusText = "HxPosed Requirements";
        public string StatusText
        {
            get => _statusText;
            set
            {
                _statusText = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(StatusText)));
            }
        }

        private string _descriptorText = string.Empty;
        public string DescriptorText
        {
            get
            {
                if (_descriptorText != string.Empty)
                    return _descriptorText;

                if (VTxSeverity == InfoBarSeverity.Error)
                    return "You need a new CPU.";
                else if (WindowsSeverity == InfoBarSeverity.Error)
                    return "Your Windows version is not supported";
                else if (UefiSeverity == InfoBarSeverity.Error)
                    return "You are using legacy boot or your system is still BIOS";
                else if (SecureBootSeverity == InfoBarSeverity.Warning || AdminPrivilegesSeverity == InfoBarSeverity.Warning)
                    return "You are good to go with a few adjustments.";

                CanInstall = true;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(Icon)));
                return "Everything is good to go!";
            }
            set
            {
                _descriptorText = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(DescriptorText)));
            }
        }

        private SymbolRegular _icon = SymbolRegular.Empty;
        public SymbolRegular Icon
        {
            get
            {
                if (_icon != SymbolRegular.Empty)
                    return _icon;

                return CanInstall ? SymbolRegular.Checkmark24 : SymbolRegular.Warning24;
            }
            set
            {
                _icon = value;
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(Icon)));
            }
        }

        public event PropertyChangedEventHandler? PropertyChanged;
    }
}
