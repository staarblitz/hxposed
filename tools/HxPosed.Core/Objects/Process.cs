using HxPosed.Core.Types;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.ComponentModel;
using System.Diagnostics.Contracts;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Objects
{
    public class Process : IObject
    {
        // no constructors!
        private Process() { }
        ~Process() => Dispose();
        private bool _disposed = false;

        // maybe use SafeHandles?

        public static Process FromId(int id)
        {
            var me = new Process
            {
                Object = HxPosed.OpenObject(ServiceFunction.OpenProcess, id),
            };

            me.ExeName = HxPosed.GetProcessNtPath(me.Object);

            foreach (var thread in HxPosed.GetProcessThreaads(me.Object))
            {
                // the threads might have changed :/
                try
                {
                    me.Threads.Add(Thread.FromId(thread, id));
                }
                catch
                {

                }
            }

            return me;
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
            var thread = Win32.OpenProcess(Win32.PROCESS_QUERY_INFORMATION, false, Win32.GetCurrentProcessId());
            if (thread == nint.Zero) throw new Win32Exception();

            Handle.SwapAndUpgrade(thread, Object);

            return thread;
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

        private void SetMitigationFlags(ProcessMitigationFlags value)
        {
            HxPosed.HxSetProcessMitigation(Object, ref value).ThrowIfError();
        }

        private ProcessMitigationFlags GetMitigationFlags()
        {
            var mitigation = new ProcessMitigationFlags();
            HxPosed.HxGetProcessMitigation(Object, ref mitigation).ThrowIfError();
            return mitigation;
        }

        private void SetSigners(ProcessSigners value)
        {
            HxPosed.HxSetProcessSigners(Object, ref value).ThrowIfError();
        }

        private ProcessSigners GetSigners()
        {
            var signers = new ProcessSigners();
            HxPosed.HxGetProcessSigners(Object, ref signers).ThrowIfError();
            return signers;
        }

        private Token GetPrimaryToken()
        {
            var token = nint.Zero;
            HxPosed.HxGetProcessToken(Object, ref token).ThrowIfError();
            return Token.FromRaw(token);
        }

        private void SetPrimaryToken()
        {
            var temp = PrimaryToken.Object;
            HxPosed.HxSetProcessToken(Object, ref temp).ThrowIfError();
        }

        public int Id { get; private set; }
        public string ExeName { get; private set; }

        public Token PrimaryToken
        {
            get => GetPrimaryToken();
            set => SetPrimaryToken();
        }

        public ProcessProtection Protection
        {
            get => GetProtection();
            set => SetProtection(value);
        }

        public ProcessMitigationFlags Mitigation
        {
            get => GetMitigationFlags();
            set => SetMitigationFlags(value);
        }

        public ProcessSigners Signers
        {
            get => GetSigners();
            set => SetSigners(value);
        }

        public ObservableCollection<Thread> Threads { get; set; } = [];

        public HxProcess Object { get; private set; }
    }
}
