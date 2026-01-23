use core::ptr::null_mut;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::nt::callback::NtCallback;

use crate::nt::mm::mdl::MemoryDescriptor;
use crate::nt::process::NtProcess;
use crate::nt::thread::NtThread;
use crate::nt::token::NtToken;
use crate::utils::alloc::PoolAlloc;
use crate::utils::danger::DangerPtr;
use alloc::boxed::Box;
use alloc::collections::VecDeque;
use hashbrown::HashMap;
use hxposed_core::hxposed::{CallbackObject, MdlObject, ProcessObject, ThreadObject, TokenObject};
use spin::mutex::SpinMutex;
use spin::Once;
use wdk_sys::KEVENT;

pub(crate) mod async_obj;

// reserved for guests
static GLOBAL_CALLBACKS: SpinMutex<Once<HashMap<CallbackObject, NtCallback>>> =
    SpinMutex::new(Once::new());
static GLOBAL_THREADS: SpinMutex<Once<HashMap<ThreadObject, NtThread>>> =
    SpinMutex::new(Once::new());
static GLOBAL_MDLS: SpinMutex<Once<HashMap<MdlObject, MemoryDescriptor>>> =
    SpinMutex::new(Once::new());
static GLOBAL_PROCESSES: SpinMutex<Once<HashMap<ProcessObject, NtProcess>>> =
    SpinMutex::new(Once::new());
static GLOBAL_TOKENS: SpinMutex<Once<HashMap<TokenObject, NtToken>>> = SpinMutex::new(Once::new());

#[repr(C)]
pub struct ObjectTracker {
    pub callbacks: HashMap<CallbackObject, NtCallback>,
    pub threads: HashMap<ThreadObject, NtThread>,
    pub tokens: HashMap<TokenObject, NtToken>,
    pub processes: HashMap<ProcessObject, NtProcess>,
    pub mdls: HashMap<MdlObject, MemoryDescriptor>,
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
        me.mdls = HashMap::new();

        me.ptr
    }

    pub fn add_mdl(&mut self, mdl: MemoryDescriptor) {
        self.mdls.insert(mdl.mdl.ptr as _, mdl);
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

    pub fn get_allocated_mdl(&mut self, mdl_addr: MdlObject) -> Option<&mut MemoryDescriptor> {
        self.mdls.get_mut(&mdl_addr)
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

    pub fn pop_allocated_mdl(&mut self, mdl: MdlObject) -> Option<MemoryDescriptor> {
        self.mdls.remove(&mdl)
    }

    pub fn pop_open_token(&mut self, token: ProcessObject) -> Option<NtToken> {
        self.tokens.remove(&token)
    }

    pub fn pop_open_callback(&mut self, callback: CallbackObject) -> Option<NtCallback> {
        self.callbacks.remove(&callback)
    }
}
