using HxPosed.GUI.Models;
using HxPosed.GUI.ViewModels;
using HxPosed.Plugins;
using HxPosed.Plugins.Config;
using HxPosed.Plugins.Permissions;
using Microsoft.Win32;
using System.Diagnostics;
using System.IO;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using Wpf.Ui;
using Wpf.Ui.Controls;
using Wpf.Ui.Extensions;

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

            Plugin.New(guid, nameTxt.Text, descTxt.Text, ver, urlTxt.Text, authTxt.Text, iconTxt.Text, pathText.Text, PluginPermissions.None);

            var pageCtx = DataContext;
            DataContext = null;
            DataContext = pageCtx;
        }

        private CancellationTokenSource _cts = new();

        private void CardAction_Click(object sender, RoutedEventArgs e)
        {
            // weird wpf com dialog shit
            new Thread(async () =>
            {
                //var cfg = new PluginConfig
                //{
                //    Revision = 0,
                //    Author = "Staarblitz",
                //    Name = "HxTest",
                //    Description = "Plugin to activate check_hv_vendor.exe",
                //    Downloads = [],
                //    Guid = Guid.Parse("ca170835-4a59-4c6d-a04b-f5866f592c38"),
                //    Icon = "AddSubtract24",
                //    Path = "Z:\\debug\\check_hv_vendor.exe",
                //    Permissions = PluginPermissions.ProcessControl | PluginPermissions.CpuIO,
                //    Url = "https://github.com/staarblitz/hxposed",
                //    Version = 1
                //};

                //JsonSerializer.Serialize(cfg);

                var dlg = new OpenFileDialog
                {
                    Filter = "HxPosed Plugin Config Files (.hpc)|*.hpc",
                    Multiselect = false,
                    Title = "Select a file",
                    InitialDirectory = "C:\\"
                };

                if (dlg.ShowDialog() is not true)
                    return;

                using var fs = File.OpenRead(dlg.FileName);
                var config = await JsonSerializer.DeserializeAsync<PluginConfig>(fs);
                if (config == null)
                {
                    await Application.Current.Dispatcher.InvokeAsync(async () =>
                    {
                        await App.ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                        {
                            Title = "Malformed Config",
                            Content = "This config is malformed and cannnot be loaded",
                            CloseButtonText = "Ok"
                        });
                    });
                   
                    return;
                }

                var fuckingLock = new SemaphoreSlim(1, 1);

                if (config.Downloads.Count > 0)
                {
                    fuckingLock.Wait(); // we can enter it right away
                    await Application.Current.Dispatcher.InvokeAsync(async () =>
                    {
                        switch(await App.ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                        {
                            Title = "HxPosed Plugins",
                            Content = "This plugin contains downloads. Which can harm your computer. Continue?",
                            CloseButtonText = "Cancel",
                            PrimaryButtonText = "No",
                            SecondaryButtonText = "Continue"
                        }))
                        {
                            case ContentDialogResult.None:
                            case ContentDialogResult.Primary:
                                return;
                        }

                        fuckingLock.Release();
                    });
                }

                await fuckingLock.WaitAsync();

                Application.Current.Dispatcher.Invoke(() =>
                {
                    var ctx = DataContext as PluginsViewModel;
                    ctx.IsLoading = true;
                });

                await PluginManager.LoadFromConfig(config, _cts.Token).ContinueWith((_)=>
                {
                    Application.Current.Dispatcher.Invoke(() =>
                    {
                        var ctx = DataContext;
                        DataContext = null;
                        DataContext = ctx;
                    });
                }).ContinueWith(async (_) =>
                {
                    await Application.Current.Dispatcher.Invoke(async () =>
                    {
                        switch (await App.ContentDialogService.ShowSimpleDialogAsync(new SimpleContentDialogCreateOptions
                        {
                            Title = "HxPosed Plugins",
                            Content = "For changes to take effect, you must restart your computer. Restart now?",
                            CloseButtonText = "Cancel",
                            PrimaryButtonText = "Yes",
                        }))
                        {
                            case ContentDialogResult.Primary:
                                Process.Start("shutdown", "/r /t 0");
                                break;
                        }
                    });
                });
            }).Start();

        }
    }
}
