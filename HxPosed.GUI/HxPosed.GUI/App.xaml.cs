using HxPosed.Plugins;
using System.Configuration;
using System.Data;
using System.Windows;
using Wpf.Ui.Appearance;

namespace HxPosed.GUI
{
    /// <summary>
    /// Interaction logic for App.xaml
    /// </summary>
    public partial class App : Application
    {
        public App()
        {
            PluginManager.HealthCheck();
            DispatcherUnhandledException += (sender, exception) =>
            {
                MessageBox.Show(exception.Exception.ToString());
                exception.Handled = true;
            };
            AppDomain.CurrentDomain.UnhandledException += (sender, exception) =>
            {
                MessageBox.Show(((Exception)exception.ExceptionObject).ToString());
            };
            TaskScheduler.UnobservedTaskException += (sender, exception) =>
            {
                MessageBox.Show(exception.Exception.ToString());
                exception.SetObserved();
            };
        }
    }

}
