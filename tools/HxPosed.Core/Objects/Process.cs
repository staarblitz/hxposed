using HxPosed.Core.Types;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Objects
{
    public class Process : IDisposable
    {
        // no constructors!
        private Process() { }

        private bool _disposed = false;

        public static Process? FromId(int id)
        {
            var me = new Process();

            me.Object = HxPosed.OpenObject(ServiceFunction.OpenProcess, id);

            var tempMe = Win32.OpenProcess(Win32.PROCESS_QUERY_INFORMATION, false, (int)Win32.GetCurrentProcessId());
            var result = HxPosed.HxSwapHandleObject((ulong)tempMe, 0, me.Object);
            if (result.ErrorCode != 0) goto cleanup;
            result = HxPosed.HxUpgradeHandle((ulong)tempMe, 0, Win32.HANDLE_ALL_ACCESS);
            if (result.ErrorCode != 0) goto cleanup;

            me.Handle = tempMe;

            me.ExeName = HxPosed.GetProcessNtPath(me.Object);

            foreach (var thread in HxPosed.GetProcessThreaads(me.Object))
            {
                me.Threads.Add(new Thread
                {
                    Id = (int)thread,
                    ProcessId = me.Id,
                });
            }


            return me;

        cleanup:
            me.Dispose();
            return null;
        }

        /// <summary>
        /// Disposes the object
        /// </summary>
        public void Kill()
        {
            var result = Win32.TerminateProcess(Handle, 0);
            Dispose();
            if (result)
            {
                throw new Win32Exception(Marshal.GetLastWin32Error());
            }
            Dispose();
        }

        public void Dispose()
        {
            if (_disposed) return;

            _disposed = true;

            // possible in early init
            if (Handle != 0)
            {
                Win32.CloseHandle(Handle);
            }
            HxPosed.CloseObject(ServiceFunction.CloseProcess, Object);

            // c# best practices
            // best practices are boilerplate in this language
            GC.SuppressFinalize(this);
        }

        ~Process()
        {
            if (!_disposed)
            {
                Dispose();
            }
        }

        private void SetProtection(ProcessProtection value)
        {
            HxPosed.HxSetProcessProtection(Object, ref value).ThrowIfError();
        }

        private ProcessProtection GetProtection()
        {
            var protection = new ProcessProtection();
            HxPosed.HxGetProcessProtection(Object, ref protection).ThrowIfError();
            return protection;
        }

        public int Id { get; private set; }
        public string ExeName { get; private set; }

        public ProcessProtection Protection
        {
            get
            {
                return GetProtection();
            }
            set
            {
                SetProtection(value);
            }
        }
        public ProcessMitigationFlags Mitigation { get; private set; }
        public ProcessSigners Signers { get; private set; }
        public ObservableCollection<Thread> Threads { get; set; } = [];

        public HxProcess Object { get; private set; }
        public nint Handle { get; private set; }
    }
}
