use crate::nt::event::NtEvent;
use crate::nt::mm::mdl::MemoryDescriptor;
use crate::utils::danger::DangerPtr;
use crate::{nt::process::NtProcess, utils::alloc::PoolAlloc};
use alloc::collections::VecDeque;
use alloc::{boxed::Box, vec::Vec};
use core::hash::{Hash, Hasher};
use hashbrown::HashMap;
use hxposed_core::hxposed::AsyncCookie;
use spin::Once;
use spin::mutex::SpinMutex;
use wdk_sys::{_MM_PAGE_PRIORITY::HighPagePriority, _MODE::UserMode, MdlMappingNoWrite, NTSTATUS};

pub static GLOBAL_ASYNC_EVENT: Once<NtEvent> = Once::new();

/// Represents an async state for a process.
#[repr(C)]
pub struct AsyncState {
    pub data_index: usize,
    pub data_system_address: &'static mut AsyncResultData,
    pub user_mdl: MemoryDescriptor,
    pub process: NtProcess,
    pub write_lock: SpinMutex<()>,
}

impl Hash for AsyncState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.process.nt_process as _);
    }
}

pub struct AsyncResultData {
    pub result_entries: [u8; 16 * 1000 * 8],
}

impl AsyncState {
    pub fn init_global() {
        GLOBAL_ASYNC_EVENT.call_once(|| NtEvent::new());
    }

    pub fn alloc_new(process: NtProcess) -> Result<*mut Self, NTSTATUS> {
        let mut me = DangerPtr {
            ptr: Box::into_raw(Self::alloc()),
        };

        let addr = unsafe { &mut *Box::into_raw(AsyncResultData::alloc()) };
        me.user_mdl = MemoryDescriptor::new_describe(
            addr as *mut _ as _,
            size_of::<AsyncResultData>() as _,
        );

        me.data_system_address = addr;
        match me.user_mdl.map(
            Some(0x20090000),
            UserMode as _,
            ((HighPagePriority as u32) | MdlMappingNoWrite) as _,
        ) {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        me.process = process;

        Ok(me.ptr)
    }

    pub fn new(process: NtProcess) -> Result<Self, NTSTATUS> {
        let mut result_data = AsyncResultData::alloc();

        let mut user_mdl = MemoryDescriptor::new_describe(
            result_data.as_mut() as *mut _ as _,
            size_of::<AsyncResultData>() as _,
        );

        match user_mdl.map(
            Some(0x20090000),
            UserMode as _,
            ((HighPagePriority as u32) | MdlMappingNoWrite) as _,
        ) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        Ok(Self {
            data_index: 0,
            data_system_address: unsafe {&mut *Box::into_raw(result_data)},
            user_mdl,
            process,
            write_lock: SpinMutex::new(()),
        })
    }

    // kys
    /*pub fn cancel(cookie: &AsyncCookie) -> Option<()> {
        match GLOBAL_ASYNC_COMMANDS.lock().remove(cookie) {
            Some(command) => {
                //bye
                Some(())
            }
            None => None,
        }
    }*/

    pub fn write_len(&mut self, len: u32) -> u64 {
        self.write_lock.lock();
        let index = unsafe {
            self.data_system_address
                .result_entries
                .as_mut_ptr()
                .byte_offset(self.data_index as _) as *mut u32
        };
        unsafe {
            core::ptr::write::<u32>(index, len);
        }

        let old_index = self.data_index as _;
        self.data_index += 4;

        old_index
    }

    pub fn write_result<T>(&mut self, src: *const T, count: usize) -> u64 {
        self.write_lock.lock();
        let index = unsafe {
            self.data_system_address
                .result_entries
                .as_mut_ptr()
                .byte_offset(self.data_index as _)
        };
        unsafe { core::ptr::copy_nonoverlapping(src, index as *mut T, count) }

        let old_index = self.data_index as _;

        self.data_index += count * size_of::<T>();

        old_index
    }
}
