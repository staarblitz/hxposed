using System;
using System.Collections.Generic;
using System.Text;

namespace HxPosed.Plugins
{
    public enum PluginStatus : uint
    {
        Ready = 0,
        Loaded = 1,
        Error = 3
    }

    public enum PluginError : uint
    {
        None = 0,
        Unknown = 1,
        CannotFindPlugin = 2,
        InvalidSignature = 3,
    }
}
