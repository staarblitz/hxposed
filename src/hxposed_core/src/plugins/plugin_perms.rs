use bitflags::bitflags;

bitflags! {
    #[derive(Default, Debug, Clone, Copy)]
    #[repr(transparent)]
    pub struct PluginPermissions: u64 {
        const NONE              = 0;
        const PROCESS_EXECUTIVE = 1 << 0;
        const PROCESS_MEMORY    = 1 << 1;
        const PROCESS_CONTROL   = 1 << 2;
        const PROCESS_SECURITY  = 1 << 3;

        const THREAD_EXECUTIVE  = 1 << 4;
        const THREAD_CONTROL    = 1 << 5;
        const THREAD_SECURITY   = 1 << 6;
        const RESERVED5         = 1 << 7;

        const MEMORY_VIRTUAL    = 1 << 8;
        const MEMORY_PHYSICAL   = 1 << 9;
        const MEMORY_ALLOCATION = 1 << 10;
        const MEMORY_PROTECT    = 1 << 11;
        const MEMORY_ISOLATE    = 1 << 12;

        const RESERVED6         = 1 << 13;
        const RESERVED7         = 1 << 14;
        const RESERVED8         = 1 << 15;
        const RESERVED9         = 1 << 16;
        const RESERVED10        = 1 << 17;

        const CPU_MSR_READ      = 1 << 18;
        const CPU_MSR_WRITE     = 1 << 19;
        const CPU_SEGMENTATION  = 1 << 20;
        const CPU_CONTROL       = 1 << 21;

        const RESERVED11        = 1 << 22;
        const RESERVED12        = 1 << 23;
        const RESERVED13        = 1 << 24;
        const RESERVED14        = 1 << 25;
        const RESERVED15        = 1 << 26;

        const SECURITY_CREATE   = 1 << 27;
        const SECURITY_MANAGE   = 1 << 28;
        const SECURITY_DELETE   = 1 << 29;
    }
}