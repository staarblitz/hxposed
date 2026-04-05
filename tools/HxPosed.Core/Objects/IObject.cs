using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Objects
{
    public interface IObject : IDisposable
    {
        nint OpenHandle();
        HxObject Object { get; }
    }
}
