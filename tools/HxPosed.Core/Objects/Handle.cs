using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Core.Objects
{
    public class Handle
    {
        public static bool TrySwapAndUpgrade(nint handle, HxObject obj, uint accessMask = Win32.HANDLE_ALL_ACCESS)
        {
            try
            {
                HxPosed.HxSwapHandleObject((ulong)handle, 0, obj).ThrowIfError();
                HxPosed.HxUpgradeHandle((ulong)handle, 0, accessMask).ThrowIfError();
                return true;
            }
            catch
            {
                return false;
            }
        }
    }
}
