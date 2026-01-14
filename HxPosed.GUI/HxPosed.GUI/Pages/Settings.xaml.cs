using HxPosed.GUI.ViewModels;
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

namespace HxPosed.GUI.Pages
{
    /// <summary>
    /// Interaktionslogik für Settings.xaml
    /// </summary>
    public partial class Settings : Page
    {
        public Settings()
        {
            InitializeComponent();
            DataContext = new SettingsViewModel();
        }

        private void CardAction_Click(object sender, RoutedEventArgs e)
        {
            App.MainWindowNavigationView.Navigate(typeof(CallerProtectionSettings));
        }
    }
}
