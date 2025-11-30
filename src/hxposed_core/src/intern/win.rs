#[cfg(feature = "usermode")]
#[link(name = "kernel32")]
unsafe extern "C" {
    #[allow(non_snake_case)]
    pub(crate) fn WaitForSingleObject(handle: u64, time: u32) -> u32;

    #[allow(non_snake_case)]
    pub(crate) fn CreateEventA(security_attributes: *mut u8, manual_reset: u32, initial_state: u32, name: *mut u8) -> u64;

    #[allow(non_snake_case)]
    pub(crate) fn CloseHandle(handle: u64);
}