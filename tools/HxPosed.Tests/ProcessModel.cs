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

            var process = 0UL;
            unsafe
            {
                var result = HxOpenObject(_HX_SERVICE_FUNCTION.HxSvcOpenProcess, (void*)id, &process);
                if (result.ErrorCode != 0)
                {
                    return null;
                }

                me.Object = process;

                var tempMe = Win32.OpenProcess(Win32.PROCESS_QUERY_INFORMATION, false, (int)Win32.GetCurrentProcessId());
                result = HxSwapHandleObject((ulong)tempMe, 0, process);
                if (result.ErrorCode != 0) goto cleanup;
                result = HxUpgradeHandle((ulong)tempMe, 0, Win32.HANDLE_ALL_ACCESS);
                if (result.ErrorCode != 0) goto cleanup;

                me.Handle = tempMe;

                ushort* name = (ushort*)0;
                HxGetProcessNtPath(process, &name);

                me.ExeName = Marshal.PtrToStringUni((nint)name)!;

                uint* threadsptr = (uint*)0;
                var count = 0u;
                HxGetProcessThreads(process, &threadsptr, &count);

                var threads = new Span<uint>(threadsptr, (int)count);
                // lets do come c#ry eh?

                foreach (var thread in threads)
                {
                    me.Threads.Add(new ThreadModel
                    {
                        Id = (int)thread,
                        ProcessId = me.Id,
                    });
                }
            }

        cleanup:
            me.Dispose();
            return null;
        }

        public void Dispose()
        {
            _disposed = true;

            // possible in early init
            if (Handle != 0)
            {
                Win32.CloseHandle((nint)Handle);
            }
            HxCloseObject(_HX_SERVICE_FUNCTION.HxSvcCloseProcess, Object);

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
            unsafe
            {
                HxSetProcessProtection(Object, &value);
            }
        }

        private _HX_PROCESS_PROTECTION GetProtection()
        {
            unsafe
            {
                var protection = new _HX_PROCESS_PROTECTION();
                HxGetProcessProtection(Object, &protection);
                return protection;
            }
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
        public nint Handle { get; private set; }
    }
}
