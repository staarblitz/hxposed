using HxPosed.GUI.Models;
using HxPosed.Plugins;
using System.Diagnostics;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using Wpf.Ui;
using Wpf.Ui.Controls;

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

        private void Button_Click_2(object sender, RoutedEventArgs e)
        {
            guidTxt.Text = Guid.NewGuid().ToString();
        }

        private void Button_Click_3(object sender, RoutedEventArgs e)
        {
            if(!Guid.TryParse(guidTxt.Text, out var guid))
            {
                var msg = new Wpf.Ui.Controls.MessageBox
                {
                    Title = "Invalid GUID",
                    Content = "Cannot parse string to GUID"
                }.ShowDialogAsync();
                return;
            }

            if(verTxt.Value is null)
            {
                var msg = new Wpf.Ui.Controls.MessageBox
                {
                    Title = "Invalid Version",
                    Content = "Version cannot be empty"
                }.ShowDialogAsync();
                return;
            }

            if (!uint.TryParse(verTxt.Value.Value.ToString(), out var ver))
            {
                var msg = new Wpf.Ui.Controls.MessageBox
                {
                    Title = "Invalid Version",
                    Content = "Cannot parse version to uint"
                }.ShowDialogAsync();
                return;
            }

            Plugin.New(guid, nameTxt.Text, descTxt.Text, ver, urlTxt.Text, authTxt.Text, iconTxt.Text);

            var pageCtx = DataContext;
            DataContext = null;
            DataContext = pageCtx;
        }
    }
}
