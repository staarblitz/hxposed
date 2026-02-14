using HxPosed.Core.Uefi;
using HxPosed.GUI.Models;
using HxPosed.GUI.ViewModels;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net.Http;
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
using Wpf.Ui.Extensions;

namespace HxPosed.GUI.Pages
{
    /// <summary>
    /// Interaktionslogik für Install.xaml
    /// </summary>
    public partial class Install : Page
    {
        private InstallViewModel _ctx = new();
        public Install()
        {
            InitializeComponent();
            DataContext = _ctx;
        }

        void SetInstallStatus(bool canInstall, string btnContent, Visibility pBarVis, SymbolRegular icon, string status, string desc)
        {
            _ctx.CanInstall = canInstall;
            btnInstall.Content = btnContent;
            _ctx.ProgressBarVisibility = pBarVis;
            _ctx.Icon = icon;
            _ctx.StatusText = status;
            _ctx.DescriptorText = desc;
            App.MainWindowNavigationView.IsEnabled = !canInstall;
        }

        private async void Button_Click(object sender, RoutedEventArgs e)
        {
            if (_ctx.StatusText == "All good!")
            {
                Process.Start("shutdown", "/r /t 0");
                return;
            }

            var success = false;

            try
            {
                SetInstallStatus(false, "Installing...", Visibility.Visible, SymbolRegular.ArrowAutofitDown24, "Installing HxPosed...", "Hold tight...");

                Partition.MountEfiPartition("U:");

                _ctx.DescriptorText = "Downloading HxPosed....";
                using var httpClient = new HttpClient();
                var files = await httpClient.GetStreamAsync("https://raw.githubusercontent.com/staarblitz/hxposed/refs/heads/main/install/HxPosed-Beta.zip");

                using var archive = new ZipArchive(files, ZipArchiveMode.Read);
                ZipArchiveEntry? entry;

                async Task ExtractToFile(ZipArchiveEntry entry, string outPath)
                {
                    await Task.Run(async () =>
                    {
                        using var zipStream = entry.Open();
                        using var fileStream = new FileStream(outPath, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None, 81920, true);
                        await zipStream.CopyToAsync(fileStream); // This is NOT async
                    });
                }

                _ctx.DescriptorText = "Extracting files...";

                if ((entry = archive.GetEntry("hxloader.efi")) is not null)
                    await ExtractToFile(entry, "U:\\HxLoader.efi");
                else throw new BadImageFormatException("Package is corrupted. HxLoader.efi is not found!");

                if ((entry = archive.GetEntry("win_hv.sys")) is not null)
                {
                    if (!Directory.Exists("U:\\EFI\\Staarblitz"))
                        Directory.CreateDirectory("U:\\EFI\\Staarblitz");

                    await ExtractToFile(entry, "U:\\EFI\\Staarblitz\\HxPosed.sys");
                }
                else throw new BadImageFormatException("Package is corrupted. win_hv.sys is not found!");

                _ctx.DescriptorText = "Adjusting your system settings...";

                // need for Get/SetFirmwareEnvironmentVariable
                Core.PInvoke.Win32.EnableSeSystemEnvironmentPrivilege();

                if (!File.Exists("U:\\EFI\\Microsoft\\Boot\\bootmgfw.old.efi"))
                {
                    // shame we dont have File.CopyAsync
                    using var sourceStream = new FileStream("U:\\EFI\\Microsoft\\Boot\\bootmgfw.efi", FileMode.Open, FileAccess.Read, FileShare.None, 81920, true);
                    using var destStream = new FileStream("U:\\EFI\\Microsoft\\Boot\\bootmgfw.old.efi", FileMode.OpenOrCreate, FileAccess.Write, FileShare.None, 81920, true);
                    await sourceStream.CopyToAsync(destStream);
                }

                _ctx.DescriptorText = "Creating boot entry...";

                await Task.Run(() =>
                {
                    EfiDevicePathProtocol? efiPt = null;
                    for (var i = 0; i < 5; i++)
                    {
                        // :D4 for padding to 0 zeros.
                        var entry = BootEntry.ReadEntry($"Boot{i:D4}");
                        if (!entry.Description.Contains("Windows"))
                            continue;

                        efiPt = entry.ProtocolList[0];
                    }

                    // actually not critical, we can construct the efi partition device path
                    // ourselves manually. but we wont.
                    if (efiPt is null)
                        throw new Exception("Failed to find Windows entry!");


                    // the first protocol points to the device containing the file path.
                    // we can safely reuse that since HxLoader.efi resides on same device as bootmgfw
                    BootEntry.NewEntry("Boot2009", "HxLoader", EfiLoadOptionAttributes.Active,
                        [efiPt, EfiDevicePathProtocol.FromDevicePath("\\HxLoader.efi")]);
                });

                _ctx.DescriptorText = "Adding to boot order...";

                await Task.Run(() =>
                {
                    var order = BootOrder.GetBootOrder();
                    order.Insert(1, 0x2009);
                    BootOrder.SetBootOrder(order);
                });

                success = true;
            }
            finally
            {
                if (!success)
                    SetInstallStatus(true, "Install", Visibility.Hidden, SymbolRegular.Prohibited24, "Installation Failed", "Something went wrong.");
                else
                    SetInstallStatus(true, "Reboot!", Visibility.Hidden, SymbolRegular.Checkmark24, "All good!", "Restart your computer. Choose HxLoader from boot options. Have fun!");

                // goodbye
                Partition.DismountEfiPartition();
            }

        }
    }
}
