using HxPosed.GUI.Models;
using System.Diagnostics;
using System.Windows;
using System.Windows.Controls;

namespace HxPosed.GUI.Pages
{
    /// <summary>
    /// Interaktionslogik für Plugins.xaml
    /// </summary>
    public partial class Plugins : Page
    {
        public Plugins()
        {
            InitializeComponent();
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            var ctx = ((Control)sender).DataContext as PluginModel;
            ctx.Plugin.Remove();

            // Evil wpf hack to refresh the entire page
            var pageCtx = DataContext;
            DataContext = null;
            DataContext = pageCtx;
        }

        private void Button_Click_1(object sender, RoutedEventArgs e)
        {
            var ctx = ((Control)sender).DataContext as PluginModel;
            Process.Start(new ProcessStartInfo
            {
                UseShellExecute = true,
                FileName = ctx.Plugin.Url
            });
        }
    }
}
