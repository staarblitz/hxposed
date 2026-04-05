using HxPosed.Core.Types;
using System.ComponentModel;

namespace HxPosed.Core.Objects
{
    public class Thread : IObject
    {
        private Thread() { }

        ~Thread() => Dispose();

        private bool _disposed = false;

        public static Thread FromId(int id, int processId)
        {
            return new Thread
            {
                Id = id,
                ProcessId = processId,
                Object = HxPosed.OpenObject(ServiceFunction.OpenThread, id),
            };
        }
        

        public void Dispose()
        {
            if (_disposed) return;
            _disposed = true;
            GC.SuppressFinalize(this);

            if (Object != nint.Zero)
            {
                HxPosed.CloseObject(ServiceFunction.CloseToken, Object);
            }
        }

        public nint OpenHandle()
        {
            var thread = Win32.OpenThread(Win32.THREAD_QUERY_INFORMATION, false, Win32.GetCurrentThreadId());
            if (thread == nint.Zero) throw new Win32Exception();

            Handle.SwapAndUpgrade(thread, Object);

            return thread;
        }

        private bool GetIsImpersonating()
        {
            var isImpersonating = false;
            HxPosed.HxGetThreadActiveImpersonationInfo(Object, ref isImpersonating).ThrowIfError();
            return isImpersonating;
        }
        private void SetIsImpersonating(bool value)
        {
            HxPosed.HxSetThreadActiveImpersonationInfo(Object, ref value).ThrowIfError();
        }

        private Token? GetImpersonationToken()
        {
            if (!IsImpersonating) return null;

            var token = nint.Zero;
            HxPosed.HxGetThreadAdjustedClientToken(Object, ref token);
            return Token.FromRaw(token);
        }

        private void SetImpersonationToken(Token? token)
        {
            if (token is null) return;

            // this is bullshit
            var temp = token.Object;
            HxPosed.HxSetThreadAdjustedClientToken(Object, ref temp).ThrowIfError();
        }

        public Token? ImpersonationToken
        {
            get => GetImpersonationToken();
            set => SetImpersonationToken(value);
        }

        public HxThread Object { get; private set; }
        public int Id { get; set; }
        public int ProcessId { get; set; }

        public bool IsImpersonating
        {
            get => GetIsImpersonating();
            set => SetIsImpersonating(value);
        }
    }
}
