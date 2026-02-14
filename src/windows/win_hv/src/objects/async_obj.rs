use crate::nt::mm::mdl::MemoryDescriptor;
use crate::win::{NtStatus, PagePriority, ProcessorMode};
use crate::nt::process::NtProcess;
use alloc::boxed::Box;
use core::hash::{Hash, Hasher};
use spin::mutex::SpinMutex;

/// Represents an async state for a process.
#[repr(C)]
pub struct AsyncState {
    pub data_index: usize,
    pub user_mdl: MemoryDescriptor,
    pub data_system_address: Box<AsyncResultData>,
    pub process: NtProcess,
    pub write_lock: SpinMutex<()>,
}

impl Hash for AsyncState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.process.nt_process as _);
    }
}

pub struct AsyncResultData {
    // ring buffer
    pub result_entries: [u8; 16 * 1000 * 8],
}

impl AsyncState {
    pub fn alloc_new(process: NtProcess) -> Result<Box<Self>, NtStatus> {
        let mut addr = unsafe { Box::<AsyncResultData>::new_zeroed().assume_init() };

        let mut me = Self {
            data_index: 0,
            user_mdl: MemoryDescriptor::new_describe_nonpaged(
                addr.as_mut() as *mut _ as _,
                size_of::<AsyncResultData>() as _,
            ),
            process,
            data_system_address: addr,
            write_lock: SpinMutex::new(()),
        };

        match me.user_mdl.map(
            Some(0x20090000),
            ProcessorMode::UserMode,
            ((PagePriority::HighPagePriority as u32) | PagePriority::NoWrite as u32) as _,
        ) {
            Ok(_) => {}
            Err(err) => {
                log::error!("Failed to map async result into 0x20090000: {}", err);
                return Err(err);
            }
        };

        Ok(Box::new(me))
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

    pub fn write_type<T>(&mut self, src: T) -> u64 {
        self.write_lock.lock();

        let calc = self.data_index + size_of::<T>();
        if calc >= self.data_system_address.result_entries.len() {
            self.data_index = 0;
        }

        let old_index = self.data_index as _;

        self.write_type_no_ring(calc, src);

        self.data_index = calc;

        old_index
    }

    pub fn write_type_no_ring<T>(&mut self, offset: usize, src: T) {
        unsafe {
            core::ptr::write(
                self.data_system_address
                    .result_entries
                    .as_mut_ptr()
                    .byte_offset(offset as _) as *mut T,
                src,
            )
        }
    }

    pub fn write_result<T>(&mut self, src: *const T, count: usize) -> u64 {
        self.write_lock.lock();

        // make sure we won't overrun the buffer
        let calc = self.data_index + count * size_of::<T>() + 4;
        if calc >= self.data_system_address.result_entries.len() {
            self.data_index = 0;
        }

        let old_index = self.data_index as _;

        let index = unsafe {
            self.data_system_address
                .result_entries
                .as_mut_ptr()
                .byte_offset(self.data_index as _)
        };
        unsafe { core::ptr::write(index as *mut u32, (count / size_of::<T>()) as _) }
        unsafe { core::ptr::copy_nonoverlapping(src, (index as *mut T).byte_offset(4), count) }

        self.data_index = calc;

        old_index
    }
}
