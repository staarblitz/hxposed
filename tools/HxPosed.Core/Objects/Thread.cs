using HxPosed.Core.Types;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Objects
{
    public class Thread : IDisposable
    {
        private Thread() { }

        private bool _disposed = false;

        public static Thread FromId(int id, int processId)
        {
            var me = new Thread
            {
                Id = id,
                ProcessId = processId,
                Object = HxPosed.OpenObject(ServiceFunction.OpenThread, id),
            };

            var handle = Win32.OpenThread(Win32.THREAD_QUERY_INFORMATION, false, Win32.GetCurrentThreadId());
            me.Handle = handle;

            if (!Objects.Handle.TrySwapAndUpgrade(handle, me.Object))
                goto cleanup;

            return me;

        cleanup:
            me.Dispose();
            return null;
        }

        private bool GetIsImpersonating()
        {
            var isImpersonating = false;
            HxPosed.HxGetThreadActiveImpersonationInfo(Object, ref isImpersonating).ThrowIfError();
            return isImpersonating;
        }

        public void Dispose()
        {
            if (_disposed) return;

            _disposed = true;
            Win32.CloseHandle(Handle);
            HxPosed.CloseObject(ServiceFunction.CloseProcess, Object);
            GC.SuppressFinalize(this);
        }

        ~Thread()
        {
            if (!_disposed)
            {
                Dispose();
            }
        }

        public HxThread Object { get; private set; }
        public nint Handle { get; private set; }
        public int Id { get; set; }
        public int ProcessId { get; set; }

        public bool IsImpersonating => GetIsImpersonating();
    }
}
