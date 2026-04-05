use crate::nt::callback::NtCallback;

use crate::nt::mm::rmd::RawMemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::nt::token::NtToken;
use crate::utils::alloc::PoolAlloc;
use crate::utils::danger::DangerPtr;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use hxposed_core::hxposed::*;
use spin::mutex::SpinMutex;

pub static CALLER_PROCESSES: SpinMutex<Vec<NtProcess>> = SpinMutex::new(Vec::new());

#[repr(C)]
pub struct ObjectTracker {
    pub callbacks: Vec<NtCallback>,
    pub threads: Vec<NtThread>,
    pub tokens: Vec<NtToken>,
    pub processes: Vec<NtProcess>,
    pub rmds: Vec<RawMemoryDescriptor>,
}

impl Drop for ObjectTracker {
    fn drop(&mut self) {
        // has to be explicitly teared down
        for rmd in &self.rmds {
            rmd.teardown();
        }
    }
}

impl ObjectTracker {
    pub fn alloc_new() -> *mut Self {
        let mut me = DangerPtr {
            ptr: Box::into_raw(Self::alloc()),
        };

        me.callbacks = Vec::new();
        me.threads = Vec::new();
        me.tokens = Vec::new();
        me.processes = vec![NtProcess::current()];
        me.rmds = Vec::new();

        me.ptr
    }

    pub fn add_caller_process(process: NtProcess) {
        CALLER_PROCESSES.lock().push(process);
    }

    pub fn pop_caller_process(ptr: ProcessObject) -> Option<NtProcess> {
        let mut lock = CALLER_PROCESSES.lock();
        if let Some(i) = lock.iter().position(|p| p.nt_process == ptr as _) {
            Some(lock.remove(i))
        } else {
            None
        }
    }

    pub fn add_rmd(&mut self, mdl: RawMemoryDescriptor) -> u64 {
        self.rmds.push(mdl);
        (self.rmds.len() - 1) as _
    }

    pub fn add_callback(&mut self, callback: NtCallback) -> u64  {
        self.callbacks.push(callback);
        (self.callbacks.len() - 1) as _
    }

    pub fn add_open_process(&mut self, process: NtProcess) -> u64 {
        self.processes.push(process);
        (self.processes.len() - 1) as _
    }

    pub fn add_open_thread(&mut self, thread: NtThread) -> u64  {
        self.threads.push(thread);
        (self.threads.len() - 1) as _
    }

    pub fn add_open_token(&mut self, token: NtToken) -> u64  {
        self.tokens.push(token);
        (self.tokens.len() - 1) as _
    }

    pub fn get_rmd(&mut self, mdl_addr: RmdObject) -> Option<&mut RawMemoryDescriptor> {
        self.rmds.get_mut(mdl_addr as usize)
    }

    pub fn get_open_thread(&mut self, thread: ThreadObject) -> Option<&mut NtThread> {
        self.threads.get_mut(thread as usize)
    }

    pub fn get_open_token(&mut self, token: TokenObject) -> Option<&mut NtToken> {
        self.tokens.get_mut(token as usize)
    }

    pub fn get_open_process(&mut self, process: ProcessObject) -> Option<&mut NtProcess> {
        self.processes.get_mut(process as usize)
    }

    pub fn get_callback(&mut self, mdl_addr: CallbackObject) -> Option<&mut NtCallback> {
        self.callbacks.get_mut(mdl_addr as usize)
    }

    pub fn pop_open_process(&mut self, process: ProcessObject) -> Option<NtProcess> {
        if (process as usize) < self.processes.len() {
            Some(self.processes.remove(process as usize))
        } else {
            None
        }
    }

    pub fn pop_open_thread(&mut self, thread: ThreadObject) -> Option<NtThread> {
        if (thread as usize) < self.threads.len() {
            Some(self.threads.remove(thread as usize))
        } else {
            None
        }
    }

    pub fn pop_rmd(&mut self, mdl: RmdObject) -> Option<RawMemoryDescriptor> {
        if (mdl as usize) < self.rmds.len() {
            Some(self.rmds.remove(mdl as usize))
        } else {
            None
        }
    }

    pub fn pop_open_token(&mut self, token: ProcessObject) -> Option<NtToken> {
        if (token as usize) < self.tokens.len() {
            Some(self.tokens.remove(token as usize))
        } else {
            None
        }
    }

    pub fn pop_open_callback(&mut self, callback: CallbackObject) -> Option<NtCallback> {
        if (callback as usize) < self.callbacks.len() {
            Some(self.callbacks.remove(callback as usize))
        } else {
            None
        }
    }
}
