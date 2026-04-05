namespace HxPosed.Core.Objects
{
    public class Handle
    {
        public static bool TrySwapAndUpgrade(nint handle, HxObject obj, uint accessMask = Win32.HANDLE_ALL_ACCESS)
        {
            try
            {
                SwapAndUpgrade(handle, obj, accessMask);
                return true;
            }
            catch
            {
                return false;
            }
        }

        public static void SwapAndUpgrade(nint handle, HxObject obj, uint accessMask = Win32.HANDLE_ALL_ACCESS)
        {
            HxPosed.HxSwapHandleObject((ulong)handle, 0, obj).ThrowIfError();
            HxPosed.HxUpgradeHandle((ulong)handle, 0, accessMask).ThrowIfError();
        }

        public static HxObject ObjectFromHandle(nint handle)
        {
            var obj = nint.Zero;
            var acc = 0U;
            HxPosed.HxGetHandleObject(handle, handle, ref obj, ref acc).ThrowIfError();
            return obj;
        }
    }
}
