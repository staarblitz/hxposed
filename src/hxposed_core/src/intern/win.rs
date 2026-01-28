#![allow(non_snake_case)]

#[cfg(feature = "usermode")]
#[link(name = "kernel32")]
unsafe extern "C" {
    pub(crate) fn WaitForSingleObject(handle: u64, time: u32) -> u32;

    pub(crate) fn GetCurrentProcessId() -> u32;

    pub(crate) fn GetCurrentThreadId() -> u32;

    pub(crate) fn CreateEventA(
        security_attributes: *mut u8,
        manual_reset: u32,
        initial_state: u32,
        name: *mut u8,
    ) -> u64;

    pub(crate) fn CloseHandle(handle: u64);

    pub(crate) fn SetEvent(handle: u64) -> u32;

    pub(crate) fn ResetEvent(handle: u64) -> u32;

    pub(crate) fn CreateThread(
        security_attributes: *mut u8,
        stack_size: usize,
        start_address: unsafe extern "C" fn(*mut u64) -> u32,
        parameter: *mut u64,
        creation_flags: u32,
        thread_id: *mut u32,
    ) -> u64;
}
