using HxPosed.PInvoke;
using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace HxPosed.Tests
{
    public class ProcessModel
    {
        public int Id { get; set; }
        public string ExeName { get; set; }
        public _HX_PROCESS_PROTECTION Protection { get; set; }
        public _HX_PROCESS_MITIGATION_FLAGS Mitigation { get; set; }
        public _HX_PROCESS_SIGNERS Signers { get; set; }
        public ObservableCollection<ThreadModel> Threads { get; set; } = [];
    }
}
