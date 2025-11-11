use wdk_sys::{HANDLE, NTSTATUS, PROCESSINFOCLASS, PULONG, PVOID, ULONG};

pub(crate) mod alloc;
pub(crate) mod macros;

#[link(name = "ntoskrnl")]
unsafe extern "C" {
    #[allow(non_snake_case)]
    pub fn ZwQueryInformationProcess(
        ProcessHandle: HANDLE,
        ProcessInformationClass: PROCESSINFOCLASS,
        ProcessInformation: PVOID,
        ProcessInformationLength: ULONG,
        ReturnLength: PULONG,
    ) -> NTSTATUS;
}
