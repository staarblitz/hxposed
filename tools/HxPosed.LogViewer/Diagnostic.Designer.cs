namespace HxPosed.LogViewer
{
    partial class Diagnostic
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            components = new System.ComponentModel.Container();
            tabControl1 = new TabControl();
            tabPage1 = new TabPage();
            listView1 = new ListView();
            columnHeader1 = new ColumnHeader();
            columnHeader4 = new ColumnHeader();
            columnHeader2 = new ColumnHeader();
            columnHeader3 = new ColumnHeader();
            panel1 = new Panel();
            listBox1 = new ListBox();
            contextMenuStrip1 = new ContextMenuStrip(components);
            copyToolStripMenuItem = new ToolStripMenuItem();
            groupBox2 = new GroupBox();
            label17 = new Label();
            button5 = new Button();
            label18 = new Label();
            button6 = new Button();
            label15 = new Label();
            button3 = new Button();
            label16 = new Label();
            button4 = new Button();
            label14 = new Label();
            button2 = new Button();
            label13 = new Label();
            button1 = new Button();
            groupBox1 = new GroupBox();
            label12 = new Label();
            label11 = new Label();
            label10 = new Label();
            label9 = new Label();
            label8 = new Label();
            label7 = new Label();
            label5 = new Label();
            label6 = new Label();
            label3 = new Label();
            label4 = new Label();
            label2 = new Label();
            label1 = new Label();
            tabControl1.SuspendLayout();
            tabPage1.SuspendLayout();
            panel1.SuspendLayout();
            contextMenuStrip1.SuspendLayout();
            groupBox2.SuspendLayout();
            groupBox1.SuspendLayout();
            SuspendLayout();
            // 
            // tabControl1
            // 
            tabControl1.Controls.Add(tabPage1);
            tabControl1.Dock = DockStyle.Fill;
            tabControl1.Location = new Point(0, 0);
            tabControl1.Name = "tabControl1";
            tabControl1.SelectedIndex = 0;
            tabControl1.Size = new Size(977, 554);
            tabControl1.TabIndex = 0;
            // 
            // tabPage1
            // 
            tabPage1.Controls.Add(listView1);
            tabPage1.Controls.Add(panel1);
            tabPage1.Location = new Point(4, 24);
            tabPage1.Name = "tabPage1";
            tabPage1.Padding = new Padding(3);
            tabPage1.Size = new Size(969, 526);
            tabPage1.TabIndex = 0;
            tabPage1.Text = "Reference Diagnostics";
            tabPage1.UseVisualStyleBackColor = true;
            // 
            // listView1
            // 
            listView1.Columns.AddRange(new ColumnHeader[] { columnHeader1, columnHeader4, columnHeader2, columnHeader3 });
            listView1.Dock = DockStyle.Fill;
            listView1.FullRowSelect = true;
            listView1.GridLines = true;
            listView1.Location = new Point(3, 127);
            listView1.Name = "listView1";
            listView1.Size = new Size(963, 396);
            listView1.TabIndex = 1;
            listView1.UseCompatibleStateImageBehavior = false;
            listView1.View = View.Details;
            // 
            // columnHeader1
            // 
            columnHeader1.Text = "Timestamp";
            columnHeader1.Width = 150;
            // 
            // columnHeader4
            // 
            columnHeader4.Text = "Action";
            columnHeader4.Width = 100;
            // 
            // columnHeader2
            // 
            columnHeader2.Text = "Old Value";
            columnHeader2.Width = 100;
            // 
            // columnHeader3
            // 
            columnHeader3.Text = "Was Owning?";
            columnHeader3.Width = 90;
            // 
            // panel1
            // 
            panel1.Controls.Add(listBox1);
            panel1.Controls.Add(groupBox2);
            panel1.Controls.Add(groupBox1);
            panel1.Dock = DockStyle.Top;
            panel1.Location = new Point(3, 3);
            panel1.Name = "panel1";
            panel1.Size = new Size(963, 124);
            panel1.TabIndex = 2;
            // 
            // listBox1
            // 
            listBox1.ContextMenuStrip = contextMenuStrip1;
            listBox1.Dock = DockStyle.Fill;
            listBox1.FormattingEnabled = true;
            listBox1.IntegralHeight = false;
            listBox1.Location = new Point(706, 0);
            listBox1.Name = "listBox1";
            listBox1.Size = new Size(257, 124);
            listBox1.TabIndex = 3;
            listBox1.SelectedIndexChanged += listBox1_SelectedIndexChanged;
            // 
            // contextMenuStrip1
            // 
            contextMenuStrip1.Items.AddRange(new ToolStripItem[] { copyToolStripMenuItem });
            contextMenuStrip1.Name = "contextMenuStrip1";
            contextMenuStrip1.Size = new Size(181, 48);
            // 
            // copyToolStripMenuItem
            // 
            copyToolStripMenuItem.Name = "copyToolStripMenuItem";
            copyToolStripMenuItem.Size = new Size(180, 22);
            copyToolStripMenuItem.Text = "Copy";
            copyToolStripMenuItem.Click += copyToolStripMenuItem_Click;
            // 
            // groupBox2
            // 
            groupBox2.Controls.Add(label17);
            groupBox2.Controls.Add(button5);
            groupBox2.Controls.Add(label18);
            groupBox2.Controls.Add(button6);
            groupBox2.Controls.Add(label15);
            groupBox2.Controls.Add(button3);
            groupBox2.Controls.Add(label16);
            groupBox2.Controls.Add(button4);
            groupBox2.Controls.Add(label14);
            groupBox2.Controls.Add(button2);
            groupBox2.Controls.Add(label13);
            groupBox2.Controls.Add(button1);
            groupBox2.Dock = DockStyle.Left;
            groupBox2.Location = new Point(285, 0);
            groupBox2.Name = "groupBox2";
            groupBox2.Size = new Size(421, 124);
            groupBox2.TabIndex = 2;
            groupBox2.TabStop = false;
            groupBox2.Text = "Analysis";
            // 
            // label17
            // 
            label17.AutoSize = true;
            label17.Location = new Point(218, 46);
            label17.Name = "label17";
            label17.Size = new Size(89, 15);
            label17.TabIndex = 34;
            label17.Text = "Delta(-) acquire";
            // 
            // button5
            // 
            button5.Location = new Point(326, 42);
            button5.Name = "button5";
            button5.Size = new Size(75, 23);
            button5.TabIndex = 33;
            button5.Text = "Analyze";
            button5.UseVisualStyleBackColor = true;
            button5.Click += button5_Click;
            // 
            // label18
            // 
            label18.AutoSize = true;
            label18.Location = new Point(218, 19);
            label18.Name = "label18";
            label18.Size = new Size(92, 15);
            label18.TabIndex = 32;
            label18.Text = "Delta(+) acquire";
            // 
            // button6
            // 
            button6.Location = new Point(326, 15);
            button6.Name = "button6";
            button6.Size = new Size(75, 23);
            button6.TabIndex = 31;
            button6.Text = "Analyze";
            button6.UseVisualStyleBackColor = true;
            button6.Click += button6_Click;
            // 
            // label15
            // 
            label15.AutoSize = true;
            label15.Location = new Point(6, 100);
            label15.Name = "label15";
            label15.Size = new Size(86, 15);
            label15.TabIndex = 30;
            label15.Text = "Delta(-) handle";
            // 
            // button3
            // 
            button3.Location = new Point(114, 96);
            button3.Name = "button3";
            button3.Size = new Size(75, 23);
            button3.TabIndex = 29;
            button3.Text = "Analyze";
            button3.UseVisualStyleBackColor = true;
            button3.Click += button3_Click;
            // 
            // label16
            // 
            label16.AutoSize = true;
            label16.Location = new Point(6, 73);
            label16.Name = "label16";
            label16.Size = new Size(89, 15);
            label16.TabIndex = 28;
            label16.Text = "Delta(+) handle";
            // 
            // button4
            // 
            button4.Location = new Point(114, 69);
            button4.Name = "button4";
            button4.Size = new Size(75, 23);
            button4.TabIndex = 27;
            button4.Text = "Analyze";
            button4.UseVisualStyleBackColor = true;
            button4.Click += button4_Click;
            // 
            // label14
            // 
            label14.AutoSize = true;
            label14.Location = new Point(6, 46);
            label14.Name = "label14";
            label14.Size = new Size(99, 15);
            label14.TabIndex = 26;
            label14.Text = "Delta(-) reference";
            // 
            // button2
            // 
            button2.Location = new Point(114, 42);
            button2.Name = "button2";
            button2.Size = new Size(75, 23);
            button2.TabIndex = 25;
            button2.Text = "Analyze";
            button2.UseVisualStyleBackColor = true;
            button2.Click += button2_Click;
            // 
            // label13
            // 
            label13.AutoSize = true;
            label13.Location = new Point(6, 19);
            label13.Name = "label13";
            label13.Size = new Size(102, 15);
            label13.TabIndex = 24;
            label13.Text = "Delta(+) reference";
            // 
            // button1
            // 
            button1.Location = new Point(114, 15);
            button1.Name = "button1";
            button1.Size = new Size(75, 23);
            button1.TabIndex = 0;
            button1.Text = "Analyze";
            button1.UseVisualStyleBackColor = true;
            button1.Click += button1_Click;
            // 
            // groupBox1
            // 
            groupBox1.Controls.Add(label12);
            groupBox1.Controls.Add(label11);
            groupBox1.Controls.Add(label10);
            groupBox1.Controls.Add(label9);
            groupBox1.Controls.Add(label8);
            groupBox1.Controls.Add(label7);
            groupBox1.Controls.Add(label5);
            groupBox1.Controls.Add(label6);
            groupBox1.Controls.Add(label3);
            groupBox1.Controls.Add(label4);
            groupBox1.Controls.Add(label2);
            groupBox1.Controls.Add(label1);
            groupBox1.Dock = DockStyle.Left;
            groupBox1.Location = new Point(0, 0);
            groupBox1.Name = "groupBox1";
            groupBox1.Size = new Size(285, 124);
            groupBox1.TabIndex = 1;
            groupBox1.TabStop = false;
            groupBox1.Text = "Statistics";
            // 
            // label12
            // 
            label12.AutoSize = true;
            label12.Location = new Point(179, 97);
            label12.Name = "label12";
            label12.Size = new Size(13, 15);
            label12.TabIndex = 23;
            label12.Text = "0";
            // 
            // label11
            // 
            label11.AutoSize = true;
            label11.Location = new Point(179, 82);
            label11.Name = "label11";
            label11.Size = new Size(13, 15);
            label11.TabIndex = 22;
            label11.Text = "0";
            // 
            // label10
            // 
            label10.AutoSize = true;
            label10.Location = new Point(179, 64);
            label10.Name = "label10";
            label10.Size = new Size(13, 15);
            label10.TabIndex = 21;
            label10.Text = "0";
            // 
            // label9
            // 
            label9.AutoSize = true;
            label9.Location = new Point(179, 49);
            label9.Name = "label9";
            label9.Size = new Size(13, 15);
            label9.TabIndex = 20;
            label9.Text = "0";
            // 
            // label8
            // 
            label8.AutoSize = true;
            label8.Location = new Point(179, 34);
            label8.Name = "label8";
            label8.Size = new Size(13, 15);
            label8.TabIndex = 19;
            label8.Text = "0";
            // 
            // label7
            // 
            label7.AutoSize = true;
            label7.Location = new Point(179, 19);
            label7.Name = "label7";
            label7.Size = new Size(13, 15);
            label7.TabIndex = 18;
            label7.Text = "0";
            // 
            // label5
            // 
            label5.AutoSize = true;
            label5.Location = new Point(6, 97);
            label5.Name = "label5";
            label5.Size = new Size(140, 15);
            label5.TabIndex = 17;
            label5.Text = "Total handle decrements:";
            // 
            // label6
            // 
            label6.AutoSize = true;
            label6.Location = new Point(6, 82);
            label6.Name = "label6";
            label6.Size = new Size(137, 15);
            label6.TabIndex = 16;
            label6.Text = "Total handle increments:";
            // 
            // label3
            // 
            label3.AutoSize = true;
            label3.Location = new Point(6, 64);
            label3.Name = "label3";
            label3.Size = new Size(118, 15);
            label3.TabIndex = 15;
            label3.Text = "Total ref decrements:";
            // 
            // label4
            // 
            label4.AutoSize = true;
            label4.Location = new Point(6, 49);
            label4.Name = "label4";
            label4.Size = new Size(115, 15);
            label4.TabIndex = 14;
            label4.Text = "Total ref increments:";
            // 
            // label2
            // 
            label2.AutoSize = true;
            label2.Location = new Point(6, 34);
            label2.Name = "label2";
            label2.Size = new Size(100, 15);
            label2.TabIndex = 13;
            label2.Text = "Total object frees:";
            // 
            // label1
            // 
            label1.AutoSize = true;
            label1.Location = new Point(6, 19);
            label1.Name = "label1";
            label1.Size = new Size(119, 15);
            label1.TabIndex = 12;
            label1.Text = "Total object acquires:";
            // 
            // Diagnostic
            // 
            AutoScaleDimensions = new SizeF(7F, 15F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(977, 554);
            Controls.Add(tabControl1);
            Name = "Diagnostic";
            Text = "Diagnostic";
            tabControl1.ResumeLayout(false);
            tabPage1.ResumeLayout(false);
            panel1.ResumeLayout(false);
            contextMenuStrip1.ResumeLayout(false);
            groupBox2.ResumeLayout(false);
            groupBox2.PerformLayout();
            groupBox1.ResumeLayout(false);
            groupBox1.PerformLayout();
            ResumeLayout(false);
        }

        #endregion

        private TabControl tabControl1;
        private TabPage tabPage1;
        private ListView listView1;
        private Panel panel1;
        private GroupBox groupBox1;
        private Label label12;
        private Label label11;
        private Label label10;
        private Label label9;
        private Label label8;
        private Label label7;
        private Label label5;
        private Label label6;
        private Label label3;
        private Label label4;
        private Label label2;
        private Label label1;
        private GroupBox groupBox2;
        private Label label13;
        private Button button1;
        private Label label17;
        private Button button5;
        private Label label18;
        private Button button6;
        private Label label15;
        private Button button3;
        private Label label16;
        private Button button4;
        private Label label14;
        private Button button2;
        private ColumnHeader columnHeader4;
        private ColumnHeader columnHeader2;
        private ColumnHeader columnHeader3;
        private ListBox listBox1;
        private ColumnHeader columnHeader1;
        private ContextMenuStrip contextMenuStrip1;
        private ToolStripMenuItem copyToolStripMenuItem;
    }
}