using HxPosed.Plugins.Config;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.GUI.Async
{
    internal class LoadFromConfigProgress
    {
        public string? CurrentFile { get; set; } = "";
        public int? RemainingFiles { get; set; } = 0;
        public CancellationToken Token { get; set; }
    }
}
