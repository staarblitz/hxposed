using HxPosed.Core.Guard;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using static HxPosed.Core.Guard.HxGuard.CallerVerificationSettings;

namespace HxPosed.GUI.ViewModels
{
    internal class CallerProtectionSettingsViewModel : INotifyPropertyChanged
    {
        public bool CallerVerificationEnabled
        {
            get { return HxGuard.CallerVerification.GetCallerVerification(); }
            set
            {
                HxGuard.CallerVerification.SetCallerVerification(value);
                PropertyChanged.Invoke(this, new PropertyChangedEventArgs(nameof(CallerVerificationEnabled)));
            }
        }

        public event PropertyChangedEventHandler? PropertyChanged;

        public ObservableCollection<VerifiedCaller> VerifiedCallers { get; } = [];

        public void GetVerifiedCallers()
        {
            VerifiedCallers.Clear();
            // why no AddRange?????
            foreach(var item in HxGuard.CallerVerification.GetVerifiedCallers())
            {
                VerifiedCallers.Add(item);
            }
        }

        public void SetVerifiedCallers()
        {
            HxGuard.CallerVerification.SetVerifiedCallers(VerifiedCallers.ToList());
            GetVerifiedCallers();
        }
    }
}
