## State call
Gets the version of HxPosed kernel driver and current system state.

### Call
Set ´RCX´ to ´0x2009´
Set ´RSI´ to ´HypervisorCall´ with function code ´0x1´
Function code: ´0x1´
All other fields are ignored.

### Return
´R8´ - ´HypervisorStatus´ (Unknown [0], SystemVirtualized [1], SystemDeVirtualized[2])
´R9´ - ´u32´ version code (1 as of writing)
´R10´ - Unused. 0
´RSI´ - ´HypervisorResult´ (Only func bit is set to ´0x1´)

´RCX´ - If anything else than ´0x2009´, hypervisor did NOT catch the trap.