using System.Collections.ObjectModel;
using System.Runtime.InteropServices;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using HxPosed.Core.Objects;
using HxPosed.Core;
using HxPosed.Core.Types;

namespace HxPosed.Tests
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public ObservableCollection<Process> Processes { get; } = [];

        public MainWindow()
        {
            InitializeComponent();
            DataContext = this; // 200 iq
            ReloadProcesses();
        }

        private void ReloadProcesses()
        {
            foreach (var process in Processes)
            {
                //bruh
                process.Dispose();
            }

            Processes.Clear();

            var status = Win32.NtQuerySystemInformation(Win32.SystemProcessInformation, nint.Zero, 0, out var returnlen);

            if (status != 0xC0000004)
            {
                MessageBox.Show($"Failed to fetch processes length! {status:x}");
                return;
            }

            var ptr = Marshal.AllocHGlobal(returnlen);

            status = Win32.NtQuerySystemInformation(Win32.SystemProcessInformation, ptr, returnlen, out var returnLength2);
            if (status != 0)
            {
                MessageBox.Show($"Failed to fetch processes length! {status:x}");
                return;
            }

            unsafe
            {
                var spi = (Win32.SYSTEM_PROCESS_INFORMATION*)ptr;
                do
                {
                    try
                    {
                        var process = Process.FromId((int)spi->UniqueProcessId);
                        if (process is not null)
                        {
                            Processes.Add(process);
                        }
                    }
                    catch
                    {

                    }

                    spi = (Win32.SYSTEM_PROCESS_INFORMATION*)nint.Add((nint)spi, (int)spi->NextEntryOffset);
                } while (spi->NextEntryOffset != 0);
            }

            Marshal.FreeHGlobal(ptr);
        }

        private void MenuItem_Click(object sender, RoutedEventArgs e)
        {
            var selectedItem = (Process)processesList.SelectedItem;

            selectedItem.Kill();
            MessageBox.Show("Process killed");

            ReloadProcesses();
        }

        private void MenuItem_Click_1(object sender, RoutedEventArgs e)
        {
            var selectedItem = (Process)processesList.SelectedItem;

            var dlg = new ProtectionView
            {
                ProtectionSigner = (ProcessProtectionSigner)selectedItem.Protection.Signer,
                ProtectionType = (ProcessProtectionType)selectedItem.Protection.Signer,
                Audit = selectedItem.Protection.Audit == 1
            };

            if (dlg.ShowDialog() == true)
            {
                // pattern matching
                selectedItem.Protection = new ProcessProtection
                {
                    Audit = (byte)(dlg.Audit ? 1 : 0),
                    Signer = (byte)dlg.ProtectionSigner,
                    Type = (byte)dlg.ProtectionType
                };
                ReloadProcesses();
            }
        }
    }
}