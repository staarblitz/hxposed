using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.PInvoke
{
    public class SafeStruct<T> : IDisposable
        where T : struct
    {
        private nint _buffer;
        private bool _disposed;

        private SafeStruct() { }

        public void Dispose()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            Marshal.FreeHGlobal(_buffer);
            _disposed = true;
        }

        /// <summary>
        /// Allocates a new SafeStruct object for use in HxPosed.
        /// </summary>
        /// <param name="data">The structure itself.</param>
        /// <remarks>Destroys <paramref name="data"/></remarks>
        /// <returns><see cref="SafeStruct{T}"/></returns>
        public SafeStruct<T> FromStructure(T data)
        {
            _buffer = Marshal.AllocHGlobal(Marshal.SizeOf<T>());
            Marshal.StructureToPtr(data, _buffer, true);

            // finally something good in C#
            return this;
        }

        /// <summary>
        /// Gets native pointer to the structure.
        /// </summary>
        /// <returns><see cref="nint"/></returns>
        public nint DangerousGetPtr()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return _buffer;
        }
    }
}
