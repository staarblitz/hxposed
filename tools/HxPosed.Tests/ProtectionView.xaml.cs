using HxPosed.PInvoke;
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
using System.Windows.Shapes;

namespace HxPosed.Tests
{
    /// <summary>
    /// Interaction logic for ProtectionView.xaml
    /// </summary>
    public partial class ProtectionView : Window
    {
        // we could have moved those to constructor. but then we would need to expose them again since we have to somehow return from a Close().
        public _HX_PROCESS_PROTECTION_TYPE ProtectionType;
        public _HX_PROCESS_PROTECTION_SIGNER ProtectionSigner;
        public bool Audit;

        public ProtectionView()
        {
            InitializeComponent();
            DataContext = this;
            typeBox.ItemsSource = Enum.GetValues<_HX_PROCESS_PROTECTION_TYPE>().Cast<_HX_PROCESS_PROTECTION_TYPE>();
            typeBox.SelectedValue = ProtectionType;
            signBox.ItemsSource = Enum.GetValues<_HX_PROCESS_PROTECTION_SIGNER>().Cast<_HX_PROCESS_PROTECTION_SIGNER>();
            signBox.SelectedValue = ProtectionSigner;
            auditBox.IsChecked = Audit;
        }

        private void Button_Click(object sender, RoutedEventArgs e)
        {
            Audit = auditBox.IsChecked ?? false;
            ProtectionType = (_HX_PROCESS_PROTECTION_TYPE)typeBox.SelectedItem;
            ProtectionSigner = (_HX_PROCESS_PROTECTION_SIGNER)signBox.SelectedItem;
            DialogResult = true;
            Close();
        }
    }
}
