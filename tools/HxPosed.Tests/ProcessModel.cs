using HxPosed.PInvoke;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Controls;
using static HxPosed.PInvoke.Methods;

namespace HxPosed.Tests
{
    // you guys want idiomatic c#? fine, here is idiomatic c#
    public class ProcessModel : IDisposable
    {
        // no constructors!
        private ProcessModel() { }

        private bool _disposed = false;

        public static ProcessModel? FromId(int id)
        {
            var me = new ProcessModel();

            var call = HxCall(_HX_SERVICE_FUNCTION.HxSvcOpenProcess);
            call.OpenObjectRequest.AddressOrId = (ulong)id;
            call.OpenObjectRequest.OpenType = HxOpenHypervisor;

            unsafe
            {
                HxpTrap(&call);
            }

            if (call.Result.ErrorCode != 0) return null;
            me.Object = call.OpenObjectResponse.Object.Object;

            call.OpenObjectRequest.AddressOrId = (ulong)id;
            call.OpenObjectRequest.OpenType = HxOpenHandle;
            unsafe
            {
                HxpTrap(&call);
            }

            if (call.Result.ErrorCode != 0)
            {
                HxClose(_HX_SERVICE_FUNCTION.HxSvcCloseProcess, me.Object);
                return null;
            }

            me.Handle = call.OpenObjectResponse.Object.Object;

            var pathCall = HxCall(_HX_SERVICE_FUNCTION.HxSvcGetProcessField);
            pathCall.GetProcessFieldRequest.Address = me.Object;
            pathCall.GetProcessFieldRequest.Data.Field = HxProcFieldNtPath;

            unsafe
            {
                HxpTrap(&call);
            }

            if (call.Result.ErrorCode != 0) goto cleanup;

            var length = HxReadAsyncResponseLength(pathCall.GetProcessFieldResponse.NtPathOffset);
            // i want to use as much c# as possible i guess
            me.ExeName = Marshal.PtrToStringUni((nint)(0x2009000 + pathCall.GetProcessFieldResponse.NtPathOffset + 4), (int)length);

            pathCall.GetProcessFieldRequest.Address = me.Object;
            pathCall.GetProcessFieldRequest.Data.Field = HxProcFieldThreads;

            unsafe
            {
                HxpTrap(&call);
            }

            if (call.Result.ErrorCode != 0) goto cleanup;

            length = HxReadAsyncResponseLength(call.GetProcessFieldRequest.Data.ThreadsOffset);
            Span<int> threads;
            // lets do come c#ry eh?
            unsafe
            {
                threads = new Span<int>((void*)(0x2009000 + pathCall.GetProcessFieldResponse.NtPathOffset + 4), (int)length);
            }

            foreach (var thread in threads)
            {
                me.Threads.Add(new ThreadModel
                {
                    Id = thread,
                    ProcessId = me.Id,
                });
            }

            pathCall.GetProcessFieldRequest.Address = me.Object;
            pathCall.GetProcessFieldRequest.Data.Field = HxProcFieldMitigationFlags;
            unsafe
            {
                HxpTrap(&call);
            }

            if (call.Result.ErrorCode != 0) goto cleanup;

            me.Protection = pathCall.GetProcessFieldResponse.Protection;

            pathCall.GetProcessFieldRequest.Address = me.Object;
            pathCall.GetProcessFieldRequest.Data.Field = HxProcFieldProtection;
            unsafe
            {
                HxpTrap(&call);
            }

            me.Mitigation = pathCall.GetProcessFieldResponse.MitigationFlags;

            if (call.Result.ErrorCode != 0) goto cleanup;

            return me;

        cleanup:
            me.Dispose();
            return null;
        }

        public void Dispose()
        {
            _disposed = true;
            Win32.CloseHandle((nint)Handle);
            HxClose(_HX_SERVICE_FUNCTION.HxSvcCloseProcess, Object);

            // c# best practices
            // best practices are boilerplate in this language
            GC.SuppressFinalize(this);
        }

        ~ProcessModel()
        {
            if (!_disposed)
            {
                Dispose();
            }
        }

        private void SetProtection(_HX_PROCESS_PROTECTION value)
        {
            var call = HxCall(_HX_SERVICE_FUNCTION.HxSvcSetProcessField);
            call.SetProcessFieldRequest.Address = Object;
            call.SetProcessFieldRequest.Data.Protection = value;
            unsafe
            {
                HxpTrap(&call);
            }
        }

        private _HX_PROCESS_PROTECTION GetProtection()
        {
            var call = HxCall(_HX_SERVICE_FUNCTION.HxSvcGetProcessField);
            call.SetProcessFieldRequest.Address = Object;
            unsafe
            {
                HxpTrap(&call);
            }

            return call.GetProcessFieldResponse.Protection;
        }

        public int Id { get; private set; }
        public string ExeName { get; private set; }

        public _HX_PROCESS_PROTECTION Protection { get
            {
                return GetProtection();
            }
            set
            {
                SetProtection(value);
            }
        }
        public _HX_PROCESS_MITIGATION_FLAGS Mitigation { get; private set; }
        public _HX_PROCESS_SIGNERS Signers { get; private set; }
        public ObservableCollection<ThreadModel> Threads { get; set; } = [];

        public ulong Object { get; private set; }
        public ulong Handle { get; private set; }
    }
}
