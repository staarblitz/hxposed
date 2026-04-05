using HxPosed.PInvoke;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using static HxPosed.PInvoke._HX_REQUEST_RESPONSE;

namespace HxPosed.LogViewer
{
    public partial class Form1 : Form
    {
        LogEntry[] _logs = null;

        public Form1()
        {
            InitializeComponent();
        }

        private void LoadLogs()
        {
            var bytes = File.ReadAllBytes(textBox1.Text);
            _logs = MemoryMarshal.Cast<byte, LogEntry>(bytes).ToArray();
            listView1.BeginUpdate();
            listView1.Items.Clear();

            toolStripStatusLabel2.Text = _logs.Length.ToString();
            toolStripStatusLabel4.Text = _logs.Count(x => x.LogType == LogType.Error).ToString();
            toolStripStatusLabel6.Text = _logs.Count(x => x.LogType == LogType.Warn).ToString();
            toolStripStatusLabel8.Text = _logs.Count(x => x.LogType == LogType.Info).ToString();
            toolStripStatusLabel10.Text = _logs.Count(x => x.LogType == LogType.Trace).ToString();

            foreach (var log in _logs)
            {
                if (log.LogType == LogType.Error && !checkBox5.Checked) continue;
                if (log.LogType == LogType.Warn && !checkBox4.Checked) continue;
                if (log.LogType == LogType.Info && !checkBox3.Checked) continue;
                if (log.LogType == LogType.Trace && !checkBox2.Checked) continue;
                if ((log.LogEvent == LogEventTag.AcquireObject
                    || log.LogEvent == LogEventTag.FreeObject
                    || log.LogEvent == LogEventTag.IncrementRefCount
                    || log.LogEvent == LogEventTag.DecrementRefCount
                    || log.LogEvent == LogEventTag.IncrementHandleCount
                    || log.LogEvent == LogEventTag.DecrementHandleCount) && !checkBox6.Checked) continue;
                if ((log.LogEvent == LogEventTag.HyperCall
                    || log.LogEvent == LogEventTag.HyperResult) && !checkBox7.Checked) continue;
                if ((log.LogEvent == LogEventTag.QueryObject
                    || log.LogEvent == LogEventTag.TrackObject
                    || log.LogEvent == LogEventTag.DetrackObject) && !checkBox8.Checked) continue;

                try
                {
                    var item = new ListViewItem(log.Processor.ToString());
                    var time = DateTime.FromFileTimeUtc((long)log.Timestamp);
                    item.SubItems.Add(time.ToString("dd/MM/yy HH:mm:ss:fffffff"));
                    item.SubItems.Add(log.LogType.ToString());

                    string eventString;

                    switch (log.LogEvent)
                    {
                        case LogEventTag.AcquireObject:
                            {
                                item.BackColor = Color.Cyan;
                                eventString = $"Acquiring internal reference to object: 0x{log.Arg1:x}, is owned: {(log.Arg2 == 0 ? false : true)}";
                                break;
                            }
                        case LogEventTag.FreeObject:
                            {
                                item.BackColor = Color.Cyan;
                                eventString = $"Releasing internal reference to object: 0x{log.Arg1:x}";
                                break;
                            }
                        case LogEventTag.QueryObject:
                            {
                                item.BackColor = Color.Cyan;
                                eventString = $"Querying object 0x{log.Arg1:x} for caller: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.TrackObject:
                            {
                                item.BackColor = Color.Cyan;
                                eventString = $"Tracking object 0x{log.Arg1:x} for caller: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.DetrackObject:
                            {
                                item.BackColor = Color.Cyan;
                                eventString = $"Untracking object 0x{log.Arg1:x} for caller: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.NoHxInfo:
                            {
                                item.BackColor = Color.Yellow;
                                eventString = "The caller doesn't have HxInfo structure";
                                break;
                            }
                        case LogEventTag.HyperCall:
                            {
                                item.BackColor = Color.LightBlue;
                                var call = new _HX_CALL
                                {
                                    _bitfield = log.Arg1
                                };

                                switch ((_HX_SERVICE_FUNCTION)call.ServiceFunction)
                                {
                                    case _HX_SERVICE_FUNCTION.HxSvcGetProcessField:
                                        {
                                            eventString = $"Hypercall to get process (0x{log.Arg2:x}) field: ({(HxProcessField)log.Arg3})";
                                            break;
                                        }
                                    case _HX_SERVICE_FUNCTION.HxSvcSetProcessField:
                                        {
                                            eventString = $"Hypercall to set process (0x{log.Arg2:x}) field: ({(HxProcessField)log.Arg3}) to: {log.Arg4}";
                                            break;
                                        }
                                    case _HX_SERVICE_FUNCTION.HxSvcGetTokenField:
                                        {
                                            eventString = $"Hypercall to get token (0x{log.Arg2:x}) field: ({(HxTokenField)log.Arg3})";
                                            break;
                                        }
                                    case _HX_SERVICE_FUNCTION.HxSvcSetTokenField:
                                        {
                                            eventString = $"Hypercall to set token (0x{log.Arg2:x}) field: ({(HxTokenField)log.Arg3})";
                                            break;
                                        }
                                    case _HX_SERVICE_FUNCTION.HxSvcGetThreadField:
                                        {
                                            eventString = $"Hypercall to get thread (0x{log.Arg2:x}) field: ({(HxThreadField)log.Arg3})";
                                            break;
                                        }
                                    case _HX_SERVICE_FUNCTION.HxSvcSetThreadField:
                                        {
                                            eventString = $"Hypercall to set thread (0x{log.Arg2:x}) field: ({(HxThreadField)log.Arg3}) to: {log.Arg4}";
                                            break;
                                        }
                                    default:
                                        eventString = $"Hypercall was requested: {(_HX_SERVICE_FUNCTION)call.ServiceFunction} 0x{log.Arg2:x} 0x{log.Arg3:x} 0x{log.Arg4:x}";
                                        break;
                                }
                                break;
                            }
                        case LogEventTag.HcDispatch:
                            {
                                item.BackColor = Color.AliceBlue;
                                eventString = $"Dispatching hypercall with category: 0x{log.Arg1:x}, function: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.HyperResult:
                            {
                                item.BackColor = Color.AliceBlue;
                                var result = Unsafe.BitCast<ulong, _HX_RESULT>(log.Arg1);
                                eventString = $"Result of hypercall: {result.ErrorCode} - {result.ErrorReason} 0x{log.Arg2:x} 0x{log.Arg3:x} 0x{log.Arg4:x}";
                                break;
                            }
                        case LogEventTag.FailedToMap:
                            {
                                item.BackColor = Color.IndianRed;
                                eventString = "Failed to map virtual address";
                                break;
                            }
                        case LogEventTag.HxPosedInit:
                            {
                                item.BackColor = Color.LightCyan;
                                eventString = $"HxPosed base: 0x{log.Arg1:x}, size: 0x{log.Arg2:x}";
                                break;
                            }
                        case LogEventTag.WindowsVersion:
                            {
                                item.BackColor = Color.LightCyan;
                                eventString = $"NT build: {log.Arg1}, UBR: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.FailedToAllocate:
                            {
                                item.BackColor = Color.IndianRed;
                                eventString = "Failed to make allocation";
                                break;
                            }
                        case LogEventTag.Exception:
                            {
                                item.BackColor = Color.OrangeRed;
                                eventString = $"Excecption with code {log.Arg1} occured";
                                break;
                            }
                        case LogEventTag.Catastrophic:
                            {
                                item.BackColor = Color.DarkRed;
                                eventString = $"Catastrophic failure. Registers: 0x{log.Arg1:x}, vector: 0x{log.Arg2:x}, code: 0x{log.Arg3:x}";
                                break;
                            }
                        case LogEventTag.IncrementHandleCount:
                            {
                                item.BackColor = Color.Gold;
                                eventString = $"Incrementing handle count of object header 0x{log.Arg1:x} old count: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.DecrementHandleCount:
                            {
                                item.BackColor = Color.PaleGoldenrod;
                                eventString = $"Decrementing handle count of object header 0x{log.Arg1:x} old count: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.DecrementRefCount:
                            {
                                item.BackColor = Color.DarkGoldenrod;
                                eventString = $"Decrementing reference count of object header 0x{log.Arg1:x} old count: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.IncrementRefCount:
                            {
                                item.BackColor = Color.PaleGoldenrod;
                                eventString = $"Incrementing reference count of object header 0x{log.Arg1:x} old count: {log.Arg2}";
                                break;
                            }
                        case LogEventTag.Panic:
                            {
                                item.BackColor = Color.DarkRed;
                                eventString = $"Panic occured. Panic info 0x{log.Arg1:x}, HxLoader info: 0x{log.Arg2:x}";
                                break;
                            }

                        case LogEventTag.NtInfo:
                            {
                                item.BackColor = Color.LightCyan;
                                eventString = $"NT build: {log.Arg1}, UBR: {log.Arg2}, base: 0x{log.Arg3:x}";
                                break;
                            }
                        case LogEventTag.BuildOffset:
                            {
                                item.BackColor = Color.LightCyan;
                                var name = log.Arg1 switch
                                {
                                    0 => "PsTerminateProcess",
                                    1 => "ExCreateHandle",
                                    2 => "PsTerminateThread",
                                    3 => "ExpLookupHandleTableEntry",
                                    _ => "Unknown"
                                };
                                eventString = $"Absolute address for {name}: 0x{log.Arg2:x}";
                                break;
                            }
                        default:
                            {
                                eventString = $"Unknown event: {(int)log.LogEvent}";
                                break;
                            }
                    }

                    item.SubItems.Add(eventString);

                    if (!checkBox1.Checked)
                    {
                        item.BackColor = Color.White;
                    }

                    listView1.Items.Add(item);
                }
                catch (Exception ex)
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
        private int _selectedSubItemIndex;
        private ListViewItem _selectedItem;

        private void copyCellToolStripMenuItem_Click(object sender, EventArgs e)
        {
            if (_selectedItem != null && _selectedSubItemIndex >= 0)
            {
                Clipboard.SetText(_selectedItem.SubItems[_selectedSubItemIndex].Text);
            }
        }

        private void copyRowToolStripMenuItem_Click(object sender, EventArgs e)
        {
            if (_selectedItem == null) return;

            var builder = new StringBuilder();
            builder.Append(_selectedItem.Text);
            foreach (ListViewItem.ListViewSubItem subitem in _selectedItem.SubItems)
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
            _selectedItem = item;
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
            _selectedSubItemIndex = subItemIndex;

            copyCellToolStripMenuItem.Enabled = true;
        }

        private void referenceDiagnosticsToolStripMenuItem_Click(object sender, EventArgs e)
        {

        }

        private void toolStripSplitButton1_ButtonClick(object sender, EventArgs e)
        {
            new Diagnostic(_logs).Show();
        }

    }
}
