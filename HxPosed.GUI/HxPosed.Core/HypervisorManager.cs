using HxPosed.Core.Exceptions;
using HxPosed.Core.Request;
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
        [DllImport("libhxposed.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern HypervisorError HxGetStatus(out HxStatus response);


        /// <summary>
        /// Gets <see cref="Status"/> struct.
        /// </summary>
        /// <exception cref="HypervisorException">Throws if hypervisor is not responding to CPUID traps.</exception>
        public static HxStatus GetHypervisorStatus()
        {
            var error = HxGetStatus(out var response);

            if (error.IsError())
                throw new HypervisorException(error);

            return response;
        }
    }
}
