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

        [Display(Name = "Process Executive", Description = "Allow executive access (e.g. kill, create, suspend) to process objects.")]
        ProcessExecutive = 1 << 0,
        [Display(Name = "Process Memory", Description = "Allow access to memory of process objects.")]
        ProcessMemory = 1 << 1,
        [Display(Name = "Process Protection", Description = "Allow access to process protection modifiers.")]
        ProcessProtection = 1 << 2,

        Reserved = 1 << 3, // ProcessEnclave?

        Reserved2 = 1 << 4,
        Reserved3 = 1 << 5,
        Reserved4 = 1 << 6,
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

        [Display(Name = "Msr Read", Description = "Allow reading model-specific-register values.")]
        CpuMsrRead = 1 << 18,
        [Display(Name = "Msr Read", Description = "Allow writing model-specific-register values.")]
        CpuMsrWrite = 1 << 19,
        [Display(Name = "Msr Read", Description = "Allow R/W segmentation registers.")]
        CpuSegmentation = 1 << 20,
        [Display(Name = "Msr Read", Description = "Allow R/W control registers.")]
        CpuControl = 1 << 21,

        Reserved11 = 1 << 22,
        Reserved12 = 1 << 23,
        Reserved13 = 1 << 24,
        Reserved14 = 1 << 25,
        Reserved15 = 1 << 26,

        [Display(Name = "Security Create", Description = "Allow creating new security tokens.")]
        SecurityCreate = 1 << 27,
        [Display(Name = "Security Manage", Description = "Allow managing existing security tokens.")]
        SecurityManage = 1 << 28,
        [Display(Name = "Security Delete", Description = "Allow deleting security tokens.")]
        SecurityDelete = 1 << 29,

        // Rest is reserved

        MaximumAllowed = ulong.MaxValue
    }
}
