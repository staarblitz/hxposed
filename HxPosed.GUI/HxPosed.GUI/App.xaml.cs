
using HxPosed.Core.Guard;
using Microsoft.Win32;
using System.Configuration;
using System.Data;
using System.Windows;
using System.Windows.Controls;
using Wpf.Ui;
using Wpf.Ui.Appearance;
using Wpf.Ui.Controls;
using Wpf.Ui.Extensions;

namespace HxPosed.GUI
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : Application
    {
        public static ContentDialogService ContentDialogService { get; } = new ContentDialogService();
        

        public static NavigationView MainWindowNavigationView
        {
            get
            {
                // not null. come on C#.
                return (Current.MainWindow as MainWindow).navView;
            }
        }

        private void CheckRegistrySanity()
        {
            if(Registry.LocalMachine.OpenSubKey("SOFTWARE\\HxPosed") == null)
            {
                Registry.LocalMachine.CreateSubKey("SOFTWARE\\HxPosed");
            }
            if(Registry.LocalMachine.OpenSubKey("SOFTWARE\\HxPosed\\HxGuard") == null)
            {
                Registry.LocalMachine.CreateSubKey("SOFTWARE\\HxPosed\\HxGuard");
                HxGuard.RegistryProtection.SetRegistryProtection(true);
                HxGuard.CallerVerification.SetCallerVerification(true);
            }
            if (Registry.LocalMachine.OpenSubKey("SOFTWARE\\HxPosed\\HxGuard\\CallerVerification") == null)
            {
                Registry.LocalMachine.CreateSubKey("SOFTWARE\\HxPosed\\HxGuard\\CallerVerification");
                HxGuard.CallerVerification.SetVerifiedCallers([]);
            }
        }

        public App()
        {
            CheckRegistrySanity();
            DispatcherUnhandledException += (sender, exception) =>
            {
                ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                {
                    Title = "Error",
                    Content = $"An error occured: {exception.Exception.Message}\n{exception.Exception}",
                    CloseButtonText = "OK"
                });
                exception.Handled = true;
            };
            AppDomain.CurrentDomain.UnhandledException += (sender, exception) =>
            {
                ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                {
                    Title = "Error",
                    Content = $"An error occured: {(exception.ExceptionObject as Exception).Message}\n{exception.ExceptionObject as Exception}",
                    CloseButtonText = "OK"
                });
            };
            TaskScheduler.UnobservedTaskException += (sender, exception) =>
            {
                ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                {
                    Title = "Error",
                    Content = $"An error occured: {exception.Exception.Message}\n{exception.Exception}",
                    CloseButtonText = "OK"
                });
                exception.SetObserved();
            };
        }
    }

}
