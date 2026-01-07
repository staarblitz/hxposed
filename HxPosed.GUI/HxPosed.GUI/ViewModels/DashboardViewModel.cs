using HxPosed.Core;
using HxPosed.Core.Exceptions;
using HxPosed.Core.Response;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Diagnostics;
using System.Text;
using Wpf.Ui.Controls;

namespace HxPosed.GUI.ViewModels
{
    internal class DashboardViewModel : INotifyPropertyChanged
    {
        private static HxStatus GetHypervisorStatus()
        {
            try
            {
                if (DesignerProperties.GetIsInDesignMode(new System.Windows.DependencyObject()))
                {
                    return new HxStatus
                    {
                        Status = HypervisorStatus.SystemVirtualized,
                        Version = 0xDEAD
                    };
                }

                return HypervisorManager.GetHypervisorStatus();
            }
            catch (HypervisorException ex)
            {
                return new HxStatus { Status = HypervisorStatus.Unknown, Version = 0 };
            }
        }

        private HxStatus _hvStatus = GetHypervisorStatus();

        public event PropertyChangedEventHandler? PropertyChanged;

        public string StatusText
        {
            get
            {
                return _hvStatus.Status switch
                {
                    HypervisorStatus.SystemVirtualized => "Locked and loaded",
                    HypervisorStatus.Unknown => "Cannot call into hypervisor",
                    HypervisorStatus.SystemDeVirtualized => "Not virtualizeed"
                };
            }
            set
            {
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(StatusText)));
            }
        }

        public string DescriptorText
        {
            get
            {
                return _hvStatus.Status switch
                {
                    HypervisorStatus.SystemVirtualized => "Everything is good to go",
                    HypervisorStatus.Unknown => "Hypervisor is not loaded",
                    HypervisorStatus.SystemDeVirtualized => "You are on your own"
                };
            }
            set
            {
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(StatusText)));
            }
        }

        public SymbolRegular Icon
        {
            get
            {
                return _hvStatus.Status switch
                {
                    HypervisorStatus.Unknown => SymbolRegular.WarningShield20,
                    HypervisorStatus.SystemVirtualized => SymbolRegular.CalendarShield24,
                    HypervisorStatus.SystemDeVirtualized => SymbolRegular.ShieldProhibited24
                };
            }
            set
            {
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(Icon)));
            }
        }
    }
}
