use crate::nt::callback::NtCallback;

use crate::nt::mm::rmd::RawMemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::nt::token::NtToken;
use crate::utils::alloc::PoolAlloc;
use crate::utils::danger::DangerPtr;
use alloc::boxed::Box;
use alloc::vec::Vec;
use hashbrown::HashMap;
use spin::mutex::SpinMutex;
use hxposed_core::hxposed::*;

pub(crate) mod async_obj;

pub static CALLER_PROCESSES: SpinMutex<Vec<NtProcess>> = SpinMutex::new(Vec::new());

#[repr(C)]
pub struct ObjectTracker {
    pub callbacks: HashMap<CallbackObject, NtCallback>,
    pub threads: HashMap<ThreadObject, NtThread>,
    pub tokens: HashMap<TokenObject, NtToken>,
    pub processes: HashMap<ProcessObject, NtProcess>,
    pub rmds: HashMap<RmdObject, RawMemoryDescriptor>,
}

impl Drop for ObjectTracker {
    fn drop(&mut self) {
        // has to be explicitly teared down
        for (_, rmd) in &self.rmds {
            rmd.teardown();
        }
    }
}

impl ObjectTracker {
    pub fn alloc_new() -> *mut Self {
        let mut me = DangerPtr {
            ptr: Box::into_raw(Self::alloc()),
        };

        me.callbacks = HashMap::new();
        me.threads = HashMap::new();
        me.tokens = HashMap::new();
        me.processes = HashMap::new();
        me.rmds = HashMap::new();

        me.ptr
    }

    pub fn add_caller_process(process: NtProcess) {
        CALLER_PROCESSES.lock().push(process);
    }

    pub fn pop_caller_process(ptr: ProcessObject) -> Option<NtProcess> {
        let mut lock = CALLER_PROCESSES.lock();
        if let Some(i) = lock.iter().position(|p| p.nt_process == ptr as _){
            Some(lock.remove(i))
        } else {
            None
        }
    }

    pub fn add_rmd(&mut self, mdl: RawMemoryDescriptor) {
        self.rmds.insert(mdl.pa.into(), mdl);
    }

    pub fn add_callback(&mut self, callback: NtCallback) {
        self.callbacks.insert(callback.callback, callback);
    }

    pub fn add_open_process(&mut self, process: NtProcess) {
        self.processes.insert(process.nt_process as _, process);
    }

    pub fn add_open_thread(&mut self, thread: NtThread) {
        self.threads.insert(thread.nt_thread as _, thread);
    }

    pub fn add_open_token(&mut self, token: NtToken) {
        self.tokens.insert(token.nt_token as _, token);
    }

    pub fn get_rmd(&mut self, mdl_addr: RmdObject) -> Option<&mut RawMemoryDescriptor> {
        self.rmds.get_mut(&mdl_addr)
    }

    pub fn get_open_thread(&mut self, thread: ThreadObject) -> Option<&mut NtThread> {
        self.threads.get_mut(&thread)
    }

    pub fn get_open_token(&mut self, token: TokenObject) -> Option<&mut NtToken> {
        self.tokens.get_mut(&token)
    }

    pub fn get_open_process(&mut self, process: ProcessObject) -> Option<&mut NtProcess> {
        self.processes.get_mut(&process)
    }

    pub fn get_callback(&mut self, mdl_addr: CallbackObject) -> Option<&mut NtCallback> {
        self.callbacks.get_mut(&mdl_addr)
    }

    pub fn pop_open_process(&mut self, process: ProcessObject) -> Option<NtProcess> {
        self.processes.remove(&process)
    }

    pub fn pop_open_thread(&mut self, thread: ThreadObject) -> Option<NtThread> {
        self.threads.remove(&thread)
    }

    pub fn pop_rmd(&mut self, mdl: RmdObject) -> Option<RawMemoryDescriptor> {
        self.rmds.remove(&mdl)
    }

    pub fn pop_open_token(&mut self, token: ProcessObject) -> Option<NtToken> {
        self.tokens.remove(&token)
    }

    pub fn pop_open_callback(&mut self, callback: CallbackObject) -> Option<NtCallback> {
        self.callbacks.remove(&callback)
    }
}
