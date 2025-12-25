using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Plugins.Config
{
    public class ConfigDownload
    {
        public string Url { get; set; } = "Url";
        public string SaveLocation { get; set; } = "Location";
        public bool ShellExecuteAfter { get; set; }
    }
}
