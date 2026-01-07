using HxPosed.Core.Exceptions;
using HxPosed.Core.Request;
using HxPosed.Core.Response;
using HxPosed.Plugins.Permissions;
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

        [DllImport("libhxposed.dll")]
        private static extern HypervisorError HxAuthenticate(ref AuthenticationRequest request, out PluginPermissions grantedPermissions);


        /// <summary>
        /// Authenticates the plugin.
        /// </summary>
        /// <param name="guid">Unique identifier of the plugin.</param>
        /// <param name="permissions">Permission mask to utilize.</param>
        /// <returns>Granted permissions. See <see cref="PluginPermissions"/></returns>
        /// <exception cref="HypervisorException"></exception>
        public static PluginPermissions Authenticate(Guid guid, PluginPermissions permissions)
        {
            var request = new AuthenticationRequest
            {
                Guid = guid,
                Permissions = permissions
            };

            var error = HxAuthenticate(ref request, out var perms);

            if (error.IsError())
                throw new HypervisorException(error);

            return perms;
        }

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
