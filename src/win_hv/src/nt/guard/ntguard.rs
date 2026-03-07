/// Since we removed PatchGuard, we have to counteract this somehow.
/// Work in progress :trollface:
pub struct NtGuard {
    pub driver_signature_check: bool,
    pub scan_cpu_interval: u64,
}
