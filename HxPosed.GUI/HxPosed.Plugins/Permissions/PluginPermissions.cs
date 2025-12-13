using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Text;

namespace HxPosed.Plugins.Permissions
{
    [Flags]
    public enum PluginPermissions : ulong
    {
        None = 0,

        [Display(Name = "Process Executive", Description = "Allow executive access (e.g. change protection, flags, mitigation) to process objects.")]
        ProcessExecutive = 1 << 0,
        [Display(Name = "Process Memory", Description = "Allow access to memory of process objects.")]
        ProcessMemory = 1 << 1,
        [Display(Name = "Process Control", Description = "Allow access to process control block (e.g. affinity). RESERVED")]
        ProcessControl = 1 << 2,
        [Display(Name = "Process Security", Description = "Allow access to process' token.")]
        ProcessSecurity = 1 << 3,

        [Display(Name = "Thread Executive", Description = "Allow executive access (e.g. suspend, terminate) to thread objects.")]
        ThreadExecutive = 1 << 4,
        [Display(Name = "Thread Control", Description = "Allow access to thread control block (e.g. wait list). RESERVED")]
        ThreadControl = 1 << 5,
        [Display(Name = "Thread Security", Description = "Allow access to thread's token.")]
        ThreadSecurity = 1 << 6,

        Reserved5 = 1 << 7,

        [Display(Name = "Virtual Memory", Description = "Allow access to virtual memory (note: this is a flag rather than a permission).")]
        MemoryVirtual = 1 << 8,
        [Display(Name = "Physical Memory", Description = "Allow access to physical memory (note: this is a flag rather than a permission).")]
        MemoryPhysical = 1 << 9,
        [Display(Name = "Memory Allocation", Description = "Allow allocation of system-wide paged or non-paged memory.")]
        MemoryAllocation = 1 << 10,
        [Display(Name = "Memory Protection", Description = "Allow protecting of system-wide or process-wide memory.")]
        MemoryProtect = 1 << 11,
        [Display(Name = "Memory Isolation", Description = "Allow isolating memory via hypervisor.")]
        MemoryIsolate = 1 << 12,

        Reserved6 = 1 << 13,
        Reserved7 = 1 << 14,
        Reserved8 = 1 << 15,
        Reserved9 = 1 << 16,
        Reserved10 = 1 << 17,

        [Display(Name = "CPU Msr Read", Description = "Allow reading model-specific-register values. RESERVED")]
        CpuMsrRead = 1 << 18,
        [Display(Name = "CPU Msr Write", Description = "Allow writing model-specific-register values. RESERVED")]
        CpuMsrWrite = 1 << 19,
        [Display(Name = "CPU Segmentation", Description = "Allow R/W segmentation registers. RESERVED")]
        CpuSegmentation = 1 << 20,
        [Display(Name = "CPU Control", Description = "Allow R/W control registers. RESERVED")]
        CpuControl = 1 << 21,
        [Display(Name = "CPU I/O", Description ="Allows access to ports. RESERVED")]
        CpuIO = 1 << 22,

        Reserved12 = 1 << 23,
        Reserved13 = 1 << 24,
        Reserved14 = 1 << 25,
        Reserved15 = 1 << 26,

        Reserved16 = 1 << 27,
        [Display(Name = "Security Manage", Description = "Allow managing existing security tokens.")]
        SecurityManage = 1 << 28,
        Reserved17 = 1 << 29,

        // Rest is reserved

        MaximumAllowed = ulong.MaxValue
    }
}
