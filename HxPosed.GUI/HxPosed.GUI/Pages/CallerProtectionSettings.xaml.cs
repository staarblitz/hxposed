using HxPosed.Core.Guard;
using HxPosed.Core.PInvoke;
using HxPosed.GUI.ViewModels;
using Microsoft.Win32;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using Wpf.Ui.Controls;
using static HxPosed.Core.Guard.HxGuard.CallerVerificationSettings;

namespace HxPosed.GUI.Pages
{
    /// <summary>
    /// Interaktionslogik für CallerProtectionSettings.xaml
    /// </summary>
    public partial class CallerProtectionSettings : Page
    {
        CallerProtectionSettingsViewModel _ctx = new();
        public CallerProtectionSettings()
        {
            InitializeComponent();
            DataContext = _ctx;
            _ctx.GetVerifiedCallers();
        }

        // TODO: fix boilerplate

        private void MenuItem_Click(object sender, RoutedEventArgs e)
        {
            _ctx.VerifiedCallers.Remove((VerifiedCaller)callersList.SelectedItem);
            _ctx.SetVerifiedCallers();
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            _ctx.VerifiedCallers.Add(VerifiedCaller.FromFilePath(Win32.DosPathToDevicePath(txtFilePath.Text)));
            _ctx.SetVerifiedCallers();
        }

        private void Button_Click_1(object sender, RoutedEventArgs e)
        {
            new Thread(() =>
            {
                var dlg = new OpenFileDialog
                {
                    Title = "Open File",
                    Multiselect = false
                };

                if (dlg.ShowDialog() == true)
                {
                    App.Current.Dispatcher.BeginInvoke(() =>
                    {
                        txtFilePath.Text = dlg.FileName;
                    });
                }
            }).Start();
        }
    }
}
