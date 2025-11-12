using HxPosed.Core.Exceptions;
using HxPosed.Core.Response;
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.Core
{
    /// <summary>
    /// The general helper class for accessing hypervisor functions.
    /// </summary>
    public class HypervisorManager
    {
        [DllImport("libhxposed.dll", EntryPoint = "get_hx_state", CallingConvention = CallingConvention.Cdecl)]
        private static extern HypervisorError GetHypervisorState(ref StatusResponse response);

        /// <summary>
        /// Gets <see cref="StatusResponse"/> struct.
        /// </summary>
        /// <exception cref="HypervisorException">Throws if hypervisor is not responding to CPUID traps.</exception>
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
