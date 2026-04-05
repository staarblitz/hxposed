using Microsoft.VisualBasic.Logging;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using static System.Windows.Forms.VisualStyles.VisualStyleElement.TaskbarClock;

namespace HxPosed.LogViewer
{
    public partial class Diagnostic : Form
    {
        private IEnumerable<AnalysisEntry> _analysis = null;
        public Diagnostic(LogEntry[] logs)
        {
            InitializeComponent();

            label7.Text = logs.Count(x => x.LogEvent == LogEventTag.AcquireObject).ToString();
            label8.Text = logs.Count(x => x.LogEvent == LogEventTag.FreeObject).ToString();

            label9.Text = logs.Count(x => x.LogEvent == LogEventTag.IncrementRefCount).ToString();
            label10.Text = logs.Count(x => x.LogEvent == LogEventTag.DecrementRefCount).ToString();

            label11.Text = logs.Count(x => x.LogEvent == LogEventTag.IncrementHandleCount).ToString();
            label12.Text = logs.Count(x => x.LogEvent == LogEventTag.DecrementHandleCount).ToString();

            _analysis = logs
              .GroupBy(log => log.Arg1)
              .Select(group => new AnalysisEntry(
                  group.Key,

                  group.Sum(log =>
                      log.LogEvent == LogEventTag.IncrementRefCount ? 1 :
                      log.LogEvent == LogEventTag.DecrementRefCount ? -1 : 0),

                  group.Sum(log =>
                      log.LogEvent == LogEventTag.IncrementHandleCount ? 1 :
                      log.LogEvent == LogEventTag.DecrementHandleCount ? -1 : 0),

                  group.Sum(log =>
                      log.LogEvent == LogEventTag.AcquireObject ? 1 :
                      log.LogEvent == LogEventTag.FreeObject ? -1 : 0),

                  [.. group.OrderBy(l => l.Timestamp)]
            ));

            button1.Text = _analysis.Count(x => x.NetRefCount > 0).ToString();
            button2.Text = _analysis.Count(x => x.NetRefCount < 0).ToString();

            button4.Text = _analysis.Count(x => x.NetHandleCount > 0).ToString();
            button3.Text = _analysis.Count(x => x.NetHandleCount < 0).ToString();

            button6.Text = _analysis.Count(x => x.NetAcquire > 0).ToString();
            button5.Text = _analysis.Count(x => x.NetAcquire < 0).ToString();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetRefCount > 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void button2_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetRefCount < 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void button4_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetHandleCount > 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void button3_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetHandleCount < 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void button6_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetAcquire < 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void button5_Click(object sender, EventArgs e)
        {
            listBox1.Items.Clear();
            foreach (var entry in _analysis.Where(x => x.NetAcquire < 0).Select(x => x.ObjectAddr))
            {
                listBox1.Items.Add($"0x{entry:x}");
            }
        }

        private void listBox1_SelectedIndexChanged(object sender, EventArgs e)
        {
            if (listBox1.SelectedItem == null) return;

            listView1.Items.Clear();

            foreach (var entry in _analysis.Where(x => x.ObjectAddr == ulong.Parse(listBox1.SelectedItem!.ToString()!.Replace("0x", ""), System.Globalization.NumberStyles.HexNumber)).SelectMany(x => x.History))
            {
                var name = entry.LogEvent switch
                {
                    LogEventTag.AcquireObject => "Internal Acquire",
                    LogEventTag.FreeObject => "Internal Free",
                    LogEventTag.IncrementRefCount => "Increment Ref",
                    LogEventTag.DecrementRefCount => "Decrement Ref",
                    LogEventTag.IncrementHandleCount => "Increment Handle",
                    LogEventTag.DecrementHandleCount => "Decrement Handle",
                    _ => null
                };

                if (name == null) return; // hmm?
                var item = new ListViewItem(DateTime.FromFileTimeUtc((long)entry.Timestamp).ToString("dd/MM/yy HH:mm:ss:fffffff"));

                item.SubItems.Add(name);

                var oldCount = entry.LogEvent switch
                {
                    LogEventTag.IncrementRefCount => entry.Arg2.ToString(),
                    LogEventTag.DecrementRefCount => entry.Arg2.ToString(),
                    LogEventTag.IncrementHandleCount => entry.Arg2.ToString(),
                    LogEventTag.DecrementHandleCount => entry.Arg2.ToString(),
                    _ => "Not applicable"
                };

                item.SubItems.Add(oldCount);

                var wasOwning = entry.LogEvent switch
                {
                    LogEventTag.AcquireObject => (entry.Arg2 == 1 ? true : false).ToString(),
                    _ => "Not applicable"
                };
                item.SubItems.Add(wasOwning);

                listView1.Items.Add(item);
            }
        }

        private void copyToolStripMenuItem_Click(object sender, EventArgs e)
        {
            if (listBox1.SelectedItem is null) return;
            Clipboard.SetText(listBox1.SelectedItem.ToString()!);
        }
    }

    public record AnalysisEntry(
        ulong ObjectAddr,
        int NetRefCount,
        int NetHandleCount,
        int NetAcquire,
        List<LogEntry> History
    );
}
