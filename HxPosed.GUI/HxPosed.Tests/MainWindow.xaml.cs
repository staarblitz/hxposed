using HxPosed.PInvoke;
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
using static HxPosed.PInvoke.Methods;

namespace HxPosed.Tests
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public ObservableCollection<ProcessModel> Processes { get; } = [];

        public MainWindow()
        {
            InitializeComponent();
            DataContext = this; // 200 iq
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
                        EventHandle = (void*)CreateEvent(nint.Zero, true, false, null),
                        ObjectType = new _HX_OBJECT_TYPE
                        {
                            Object = (void*)(ulong)_HX_OBJECT_TYPES.HxObProcess
                        }
                    },
                };

                if (HxpTrap(&reqResp) != 0)
                {
                    MessageBox.Show("Hypervisor is not loaded!");
                    return;
                    //Application.Current.Shutdown();
                }

                if (reqResp.Result.ErrorCode != 0)
                {
                    MessageBox.Show($"Error registering callbacks opReq: {reqResp.Result.ErrorCode} {reqResp.Result.ErrorReason}");
                }
            }

            var status = NtQuerySystemInformation(SystemProcessInformation, nint.Zero, 0, out var returnlen);

            if (status != 0xC0000004)
            {
                MessageBox.Show($"Failed to fetch processes length! {status:x}");
                return;
            }

            var ptr = Marshal.AllocHGlobal(returnlen);

            status = NtQuerySystemInformation(SystemProcessInformation, ptr, returnlen, out var returnLength2);
            if (status != 0)
            {
                MessageBox.Show($"Failed to fetch processes length! {status:x}");
                return;
            }

            unsafe
            {
                var openProcess = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcOpenProcess
                    },
                    OpenObjectRequest = new _HXR_OPEN_OBJECT {
                        AddressOrId = 0,
                        OpenType = HxOpenHypervisor
                    }
                };

                var closeProcess = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcCloseProcess
                    },
                };

                var getMitigations = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcGetProcessField
                    },
                    GetProcessFieldRequest = new _HXR_GET_PROCESS_FIELD {
                        Data = new _HXS_GET_PROCESS_FIELD
                        {
                            Field = HxProcFieldMitigationFlags,
                        }
                    }
                };

                var getProtection = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcGetProcessField
                    },
                    GetProcessFieldRequest = new _HXR_GET_PROCESS_FIELD
                    {
                        Data = new _HXS_GET_PROCESS_FIELD
                        {
                            Field = HxProcFieldProtection,
                        }
                    }
                };

                var getPath = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcGetProcessField,
                    },
                    GetProcessFieldRequest = new _HXR_GET_PROCESS_FIELD
                    {
                        Data = new _HXS_GET_PROCESS_FIELD
                        {
                            Field = HxProcFieldNtPath
                        }
                    }
                };

                var getThreads = new _HX_REQUEST_RESPONSE
                {
                    Call = new _HX_CALL
                    {
                        ServiceFunction = (ulong)_HX_SERVICE_FUNCTION.HxSvcGetProcessField,
                    },
                    GetProcessFieldRequest = new _HXR_GET_PROCESS_FIELD
                    {
                        Data = new _HXS_GET_PROCESS_FIELD
                        {
                            Field = HxProcFieldThreads
                        }
                    }
                };

                var spi = (SYSTEM_PROCESS_INFORMATION*)ptr;
                var iter = 0;
                do {
                    iter++;
                    // yes we skip the first one on purpose


                    var opReq = openProcess;

                    opReq.OpenObjectRequest.AddressOrId = (ulong)spi->UniqueProcessId;

                    HxpTrap(&opReq);

                    if (opReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending opReq: {opReq.Result.ErrorCode} {opReq.Result.ErrorReason}");
                    }

                    var cpReq = closeProcess;
                    
                    var processAddr = opReq.OpenObjectResponse.Object.Object;
                    cpReq.CloseObjectRequest.Address = (ulong)processAddr;

                    // clone structs so original ones stay intact
                    var gmReq = getMitigations;
                    var gpReq = getProtection;
                    var gnReq = getPath;
                    var gtReq = getThreads;

                    gmReq.GetProcessFieldRequest.Address = processAddr;
                    gpReq.GetProcessFieldRequest.Address = processAddr;
                    gnReq.GetProcessFieldRequest.Address = processAddr;
                    gtReq.GetProcessFieldRequest.Address = processAddr;

                    HxpTrap(&gmReq);
                    if (gmReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending gmReq: {gmReq.Result.ErrorCode} {gmReq.Result.ErrorReason}");
                    }
                    HxpTrap(&gpReq);
                    if (gpReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending gpReq: {gpReq.Result.ErrorCode} {gpReq.Result.ErrorReason}");
                    }


                    var process = new ProcessModel
                    {
                        Id = (int)spi->UniqueProcessId,
                        Mitigation = gmReq.GetProcessFieldResponse.MitigationFlags,
                        Protection = gpReq.GetProcessFieldResponse.Protection,
                    };

                    HxpTrap(&gnReq);
                    if (gnReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending gnReq: {gnReq.Result.ErrorCode} {gnReq.Result.ErrorReason}");
                    }

                    var count = 0u;
                    var name = HxReadAsyncResponseSlice(gnReq.GetProcessFieldResponse.NtPathOffset, &count);

                    process.ExeName = Marshal.PtrToStringUni((nint)name, (int)count);

                    HxpTrap(&gtReq);
                    if (gtReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending gtReq: {gtReq.Result.ErrorCode} {gtReq.Result.ErrorReason}");
                    }

                    var threads = (uint*)HxReadAsyncResponseSlice(gtReq.GetProcessFieldResponse.ThreadsOffset, &count);
                    for (var i = 0; i < count; i++)
                    {
                        process.Threads.Add(new ThreadModel
                        {
                            Id = (int)*(threads + i),
                            ProcessId = process.Id
                        });
                    }

                    Processes.Add(process);

                    HxpTrap(&cpReq);
                    if (cpReq.Result.ErrorCode != 0)
                    {
                        MessageBox.Show($"Error sending cpReq: {opReq.Result.ErrorCode} {opReq.Result.ErrorReason}");
                    }

                    spi = (SYSTEM_PROCESS_INFORMATION*)nint.Add((nint)spi, (int)spi->NextEntryOffset);
                } while (spi->NextEntryOffset != 0);
            }

            Marshal.FreeHGlobal(ptr);
         }
    }
}