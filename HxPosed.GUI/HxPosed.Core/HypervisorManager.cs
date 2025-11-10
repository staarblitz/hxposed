using HxPosed.Core.Exceptions;
using HxPosed.Core.Response;
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core
{
    public class HypervisorManager
    {
        [DllImport("hxposed_core.dll", EntryPoint = "get_hx_state", CallingConvention = CallingConvention.Cdecl)]
        private static extern HypervisorError GetHypervisorState(ref StatusResponse response);

        public static StatusResponse GetHypervisorStatus()
        {
            var response = new StatusResponse();
            var error = GetHypervisorState(ref response);

            if (error.IsError())
                throw new HypervisorException(error);

            return response;
        }
    }
}
