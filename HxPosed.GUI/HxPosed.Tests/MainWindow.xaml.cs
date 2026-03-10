using HxPosed.PInvoke;
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
using static HxPosed.PInvoke.Methods;

namespace HxPosed.Tests
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            unsafe
            {
                var reqResp = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcRegisterNotifyEvent,
                    },
                    RegisterCallbackRequest = new _HXR_REGISTER_CALLBACK
                    {
                        EventHandle = (void*)CreateEvent(nint.Zero, true, false, null)
                    },
                };

                if (HxpTrap(&reqResp) != 0)
                {
                    MessageBox.Show("Hypervisor is not loaded!");
                    //Application.Current.Shutdown();
                }
            }
        }
    }
}