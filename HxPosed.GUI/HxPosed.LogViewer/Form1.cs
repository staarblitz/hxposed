using HxPosed.PInvoke;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;

namespace HxPosed.LogViewer
{
    public partial class Form1 : Form
    {
        LogEntry[] logs = null;

        public Form1()
        {
            InitializeComponent();
        }

        private void LoadLogs()
        {
            var bytes = File.ReadAllBytes(textBox1.Text);
            logs = MemoryMarshal.Cast<byte, LogEntry>(bytes).ToArray();
            listView1.BeginUpdate();
            listView1.Items.Clear();

            toolStripStatusLabel2.Text = logs.Length.ToString();
            toolStripStatusLabel4.Text = logs.Count(x => x.LogType == LogType.Error).ToString();
            toolStripStatusLabel6.Text = logs.Count(x => x.LogType == LogType.Warn).ToString();
            toolStripStatusLabel8.Text = logs.Count(x => x.LogType == LogType.Info).ToString();
            toolStripStatusLabel10.Text = logs.Count(x => x.LogType == LogType.Trace).ToString();

            foreach (var log in logs)
            {
                if (log.LogType == LogType.Error && !checkBox5.Checked) continue;
                if (log.LogType == LogType.Warn && !checkBox4.Checked) continue;
                if (log.LogType == LogType.Info && !checkBox3.Checked) continue;
                if (log.LogType == LogType.Trace && !checkBox2.Checked) continue;

                try
                {
                    var item = new ListViewItem(log.Processor.ToString());
                    var time = DateTime.FromFileTimeUtc((long)log.Timestamp);
                    item.SubItems.Add(time.ToString("dd/MM/yy HH:mm:ss:fffffff"));
                    item.SubItems.Add(log.LogType.ToString());

                    string eventString;

                    switch (log.LogEvent)
                    {
                        case LogEventTag.None:
                            {
                                eventString = "Invalid event";
                                break;
                            }
                        case LogEventTag.VmxExitReason:
                            {
                                eventString = $"VmExit with reason: 0x{log.Arg1:x}";
                                break;
                            }
                        case LogEventTag.RIP:
                            {
                                eventString = $"VmExit RIP: 0x{log.Arg1:x}, Next RIP: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.VCPU:
                            {
                                eventString = $"VCPU structure address: 0x{log.Arg1:x}";
                                break;
                            }
                        case LogEventTag.UnknownExitReason:
                            {
                                eventString = $"HxPosed encountered an unhandled exit reason {log.Arg1}.";
                                break;
                            }
                        case LogEventTag.NoHxInfo:
                            {
                                eventString = "The caller doesn't have HxInfo structure";
                                break;
                            }
                        case LogEventTag.HyperCall:
                            {
                                var call = new _HX_CALL
                                {
                                    _bitfield = log.Arg1
                                };

                                eventString = $"Hypercall was requested: {(_HX_SERVICE_FUNCTION)call.ServiceFunction} 0x{log.Arg2:x} 0x{log.Arg3:x} 0x{log.Arg4:x}";
                                break;
                            }
                        case LogEventTag.HcDispatch:
                            {
                                eventString = $"Dispatching hypercall with category: 0x{log.Arg1:x}, function: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.CallError:
                            {
                                eventString = "Dispatch ended";
                                break;
                            }
                        case LogEventTag.HyperResult:
                            {
                                var result = Unsafe.BitCast<ulong, _HX_RESULT>(log.Arg1);
                                eventString = $"Result of hypercall: {result.ErrorCode} - {result.ErrorReason} 0x{log.Arg2:x} 0x{log.Arg3:x} 0x{log.Arg4:x}";
                                break;
                            }
                        case LogEventTag.VirtualizingProcessor:
                            {
                                eventString = $"Processor {log.Arg1} is being virtualized";
                                break;
                            }
                        case LogEventTag.ProcessorVirtualized:
                            {
                                eventString = $"Processor {log.Arg1} virtualized";
                                break;
                            }
                        case LogEventTag.FailedToMap:
                            {
                                eventString = "Failed to map virtual address";
                                break;
                            }
                        case LogEventTag.DelayedStart:
                            {
                                eventString = "Loaded from HxLoader. Delaying start...";
                                break;
                            }
                        case LogEventTag.HxPosedInit:
                            {
                                eventString = $"HxPosed base: 0x{log.Arg1:x}, size: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.NtVersion:
                            {
                                eventString = $"NT version: {log.Arg1}";
                                break;
                            }
                        case LogEventTag.WindowsVersion:
                            {
                                eventString = $"NT build: {log.Arg1}, UBR: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.FailedToAllocate:
                            {
                                eventString = "Failed to make allocation";
                                break;
                            }
                        case LogEventTag.Exception:
                            {
                                eventString = $"Excecption with code {log.Arg1} occured";
                                break;
                            }
                        case LogEventTag.Catastrophic:
                            {
                                eventString = $"Catastrophic failure. Registers: 0x{log.Arg1:x}, vector: 0x{log.Arg2:x}, code: 0x{log.Arg3:x}";
                                break;
                            }
                        case LogEventTag.ProcessorReady:
                            {
                                eventString = $"Processor {log.Arg1} is ready with HvFs virtual address: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.LaunchingProcessor:
                            {
                                eventString = "Launching processor...";
                                break;
                            }
                        case LogEventTag.Vmclear:
                            {
                                eventString = $"Executing VMCLEAR physical: 0x{log.Arg1:x}, virtual: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.Vmptrld:
                            {
                                eventString = $"Executing VMPTRLD physical: 0x{log.Arg1:x}, virtual: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.Vmxon:
                            {
                                eventString = $"Executing VMXON physical: 0x{log.Arg1:x}, virtual: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.Panic:
                            {
                                eventString = $"Panic occured. VMCS dump: 0x{log.Arg1:x}, panic info: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.WritingAsyncBuffer:
                            {
                                eventString = $"Writing async buffer to 0x{log.Arg1:x}, with length {log.Arg2}";
                                break;
                            }
                        case LogEventTag.WrittenAsyncBuffer:
                            {
                                eventString = $"Written async buffer to 0x{log.Arg1:x}, ended at 0x{log.Arg2:x}";
                                break;
                            }
                        default:
                            {
                                eventString = "Unknown event";
                                break;
                            }
                    }

                    item.SubItems.Add(eventString);

                    if (checkBox1.Checked)
                    {
                        item.BackColor = log.LogType switch
                        {
                            LogType.Trace => item.BackColor,
                            LogType.Info => Color.CadetBlue,
                            LogType.Warn => Color.DarkGoldenrod,
                            LogType.Error => Color.IndianRed
                        };
                    }

                    listView1.Items.Add(item);
                }
                catch(Exception ex)
                {

                }
            }

            listView1.EndUpdate();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            using var dialog = new OpenFileDialog
            {
                Title = "Select log file",
                Multiselect = false,
            };

            if (dialog.ShowDialog() == DialogResult.OK)
            {
                textBox1.Text = dialog.FileName;
                LoadLogs();
            }
        }

        // insane
        private int selectedSubItemIndex;
        private ListViewItem selectedItem;

        private void copyCellToolStripMenuItem_Click(object sender, EventArgs e)
        {
            if (selectedItem != null && selectedSubItemIndex >= 0)
            {
                Clipboard.SetText(selectedItem.SubItems[selectedSubItemIndex].Text);
            }
        }

        private void copyRowToolStripMenuItem_Click(object sender, EventArgs e)
        {
            if (selectedItem == null) return;

            var builder = new StringBuilder();
            builder.Append(selectedItem.Text);
            foreach (ListViewItem.ListViewSubItem subitem in selectedItem.SubItems)
            {
                builder.Append(" - " + subitem.Text);
            }
            Clipboard.SetText(builder.ToString());
        }

        private void contextMenuStrip1_Opening(object sender, System.ComponentModel.CancelEventArgs e)
        {
            var mousePosition = listView1.PointToClient(Cursor.Position);
            var item = listView1.GetItemAt(mousePosition.X, mousePosition.Y);
            if (item is null)
            {
                e.Cancel = true;
                return;
            }
            selectedItem = item;
            var subItemIndex = -1;
            var x = 0;
            for (var i = 0; i < item.SubItems.Count; i++)
            {
                x += item.ListView.Columns[i].Width;
                if (mousePosition.X < x)
                {
                    subItemIndex = i;
                    break;
                }
            }
            selectedSubItemIndex = subItemIndex;

            copyCellToolStripMenuItem.Enabled = true;
        }
    }
}
