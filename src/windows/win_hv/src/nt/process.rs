use wdk_sys::ntddk::PsLookupProcessByProcessId;
use wdk_sys::PEPROCESS;

pub struct KernelProcess {
    eprocess: u64,
}

impl KernelProcess {
    pub fn from_id(id: u32) -> KernelProcess {
        let mut process = PEPROCESS::default();
        let status = unsafe{PsLookupProcessByProcessId(id as _, &mut process)};

        KernelProcess {
            eprocess: process as _
        }
    }
}