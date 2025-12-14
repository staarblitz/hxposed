using HxPosed.Core.Exceptions;
using HxPosed.Core.Request;
using HxPosed.Core.Response;
using Microsoft.Win32.SafeHandles;
using Nito.AsyncEx.Interop;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core
{
    public class HxProcess : IDisposable
    {
        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxOpenProcess(ref OpenProcessRequest request, out OpenObjectResponse response);
        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxCloseProcess(ref CloseProcessRequest request);
        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxGetProcessField(ref GetProcessFieldRequest request, out GetProcessFieldResponse response, ref AsyncInfo asyncInfo);
        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxGetProcessThreads(ref GetProcessThreadsRequest request, out GetProcessThreadsResponse response, ref AsyncInfo asyncInfo);
        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxSetProcessField(ref SetProcessFieldRequest request, ref AsyncInfo asyncInfo);

        public uint Id { get; private set; }

        private IntPtr _address = IntPtr.Zero;
        private bool _disposed = false;

        ~HxProcess()
        {
            Dispose();
        }

        public static HxProcess Open(uint id)
        {
            var request = new OpenProcessRequest
            {
                Id = id,
                OpenType = ObjectOpenType.Hypervisor
            };

            HxOpenProcess(ref request, out var response).ThrowIfError();

            return new HxProcess
            {
                Id = id,
                _address = response.Address,
            };
        }

        public static IntPtr OpenHandle(uint id)
        {
            var request = new OpenProcessRequest
            {
                Id = id,
                OpenType = ObjectOpenType.Hypervisor
            };

            HxOpenProcess(ref request, out var response).ThrowIfError();

            return response.Address;
        }

        public async Task<string> GetNtPath()
        {
            // C# was a mistake
            return "Use rust";
        }

        public void Dispose()
        {
            if (_disposed) return;

            var request = new CloseProcessRequest
            {
                Address = _address,
            };

            HxCloseProcess(ref request);

            _disposed = true;
        }
    }
}
